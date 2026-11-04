use env::{Mark, PieToken};
use pipe_api::{Direction, Endpoint, MAX_CAPACITY};
use shell_api::{
    Binding, Catalogue, Fail, Image, Launch, MAX_ARGS, MAX_PORTS, MAX_SIZE,
};

// Literal little-endian wire constants, independent of the library constants.
const GOLDEN_MAGIC: [u8; 8] = [0x73, 0x6c, 0x65, 0x72, 0x61, 0x77, 0x71, 0x73];
const GOLDEN_VERSION: [u8; 8] = [3, 0, 0, 0, 0, 0, 0, 0];

fn token(raw: u64) -> PieToken {
    PieToken::from_bytes(&raw.to_le_bytes()).unwrap()
}

fn word(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn text(bytes: &mut Vec<u8>, value: &str) {
    word(bytes, value.len() as u64);
    bytes.extend_from_slice(value.as_bytes());
}

fn with_word(bytes: &[u8], at: usize, value: u64) -> Vec<u8> {
    let mut out = bytes.to_vec();
    out[at..at + 8].copy_from_slice(&value.to_le_bytes());
    out
}

fn replace_text(
    bytes: &[u8],
    len_at: usize,
    start: usize,
    old_len: usize,
    new: &str,
) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len() - old_len + new.len());
    out.extend_from_slice(&bytes[..start]);
    out.extend_from_slice(new.as_bytes());
    out.extend_from_slice(&bytes[start + old_len..]);
    out[len_at..len_at + 8].copy_from_slice(&(new.len() as u64).to_le_bytes());
    out
}

fn launch_header(args: u64, ports: u64) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&GOLDEN_MAGIC);
    bytes.extend_from_slice(&GOLDEN_VERSION);
    word(&mut bytes, args);
    word(&mut bytes, ports);
    bytes
}

fn catalogue_header(images: u64) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&GOLDEN_MAGIC);
    bytes.extend_from_slice(&GOLDEN_VERSION);
    word(&mut bytes, images);
    bytes
}

// Golden fixtures below are built field by field from the fixed wire layout,
// not by the codec under test.
fn launch_fixture() -> (Launch, Vec<u8>) {
    let launch = Launch {
        args: vec!["sh".into()],
        ports: vec![
            Binding {
                name: "in".into(),
                endpoint: Endpoint {
                    id: 0x0102_0304_0506_0708,
                    capacity: 0x1000,
                    seed: token(0x21),
                    direction: Direction::Read,
                },
            },
            Binding {
                name: "out".into(),
                endpoint: Endpoint {
                    id: 2,
                    capacity: 0x1_0000,
                    seed: token(0x2a),
                    direction: Direction::Write,
                },
            },
        ],
    };
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&GOLDEN_MAGIC);
    bytes.extend_from_slice(&GOLDEN_VERSION);
    word(&mut bytes, 1);
    word(&mut bytes, 2);
    text(&mut bytes, "sh");
    text(&mut bytes, "in");
    word(&mut bytes, 0x0102_0304_0506_0708);
    word(&mut bytes, 0x1000);
    word(&mut bytes, 0x21);
    word(&mut bytes, 1);
    text(&mut bytes, "out");
    word(&mut bytes, 2);
    word(&mut bytes, 0x1_0000);
    word(&mut bytes, 0x2a);
    word(&mut bytes, 2);
    (launch, bytes)
}

fn catalogue_fixture() -> (Catalogue, Vec<u8>) {
    let catalogue = Catalogue {
        images: vec![
            Image {
                name: "emit".into(),
                seed: token(0x11),
                mark: Mark::new(0x1122_3344_5566_7788),
                length: 100,
                ports: vec![("records".into(), Direction::Write)],
            },
            Image {
                name: "cat".into(),
                seed: token(0x22),
                mark: Mark::new(0x8877_6655_4433_2211),
                length: 1,
                ports: vec![
                    ("source".into(), Direction::Read),
                    ("copy".into(), Direction::Write),
                ],
            },
        ],
    };
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&GOLDEN_MAGIC);
    bytes.extend_from_slice(&GOLDEN_VERSION);
    word(&mut bytes, 2);
    text(&mut bytes, "emit");
    word(&mut bytes, 0x11);
    word(&mut bytes, 0x1122_3344_5566_7788);
    word(&mut bytes, 100);
    word(&mut bytes, 1);
    text(&mut bytes, "records");
    word(&mut bytes, 2);
    text(&mut bytes, "cat");
    word(&mut bytes, 0x22);
    word(&mut bytes, 0x8877_6655_4433_2211);
    word(&mut bytes, 1);
    word(&mut bytes, 2);
    text(&mut bytes, "source");
    word(&mut bytes, 1);
    text(&mut bytes, "copy");
    word(&mut bytes, 2);
    (catalogue, bytes)
}

fn image(name: &str, seed: u64, length: usize, ports: Vec<(String, Direction)>) -> Image {
    Image {
        name: name.into(),
        seed: token(seed),
        mark: Mark::new(1),
        length,
        ports,
    }
}

fn max_size_launch(extra: usize) -> Launch {
    let mut args = Vec::new();
    for at in 0..MAX_ARGS {
        let len = if at == 0 {
            1016 + extra
        } else if at < MAX_ARGS / 2 {
            1016
        } else {
            1015
        };
        args.push("a".repeat(len));
    }
    Launch {
        args,
        ports: Vec::new(),
    }
}

fn max_size_catalogue(extra: usize) -> Catalogue {
    let mut images = Vec::new();
    for at in 0..32 {
        let len = if at == 0 {
            2008 + extra
        } else if at < 8 {
            2008
        } else {
            2007
        };
        let mut name = format!("{at:02}");
        name.push_str(&"x".repeat(len - 2));
        images.push(image(&name, 1, 1, Vec::new()));
    }
    Catalogue { images }
}

#[test]
fn launch_golden_wire_is_stable_in_both_directions() {
    let (launch, golden) = launch_fixture();
    assert_eq!(shell_api::MAGIC.to_le_bytes(), GOLDEN_MAGIC);
    assert_eq!(shell_api::VERSION.to_le_bytes(), GOLDEN_VERSION);
    assert_eq!(launch.encode().unwrap(), golden);

    let decoded = Launch::decode(&golden).unwrap();
    assert_eq!(decoded.args, launch.args);
    assert_eq!(decoded.ports.len(), launch.ports.len());
    for (got, want) in decoded.ports.iter().zip(&launch.ports) {
        assert_eq!(got.name, want.name);
        assert_eq!(got.endpoint.id, want.endpoint.id);
        assert_eq!(got.endpoint.capacity, want.endpoint.capacity);
        assert_eq!(got.endpoint.seed, want.endpoint.seed);
        assert_eq!(got.endpoint.direction, want.endpoint.direction);
    }
}

#[test]
fn launch_decoder_rejects_truncation_trailing_and_names() {
    let (_, golden) = launch_fixture();
    for length in 0..golden.len() {
        assert_eq!(
            Launch::decode(&golden[..length]).err(),
            Some(Fail::Invalid),
            "{length}"
        );
    }

    let mut trailing = golden.clone();
    trailing.push(0);
    assert_eq!(Launch::decode(&trailing).err(), Some(Fail::Invalid));

    let mut magic = golden.clone();
    magic[0] ^= 1;
    assert_eq!(Launch::decode(&magic).err(), Some(Fail::Invalid));

    let mut version = golden.clone();
    version[8] = 4;
    assert_eq!(Launch::decode(&version).err(), Some(Fail::Invalid));

    for at in [40, 50] {
        let mut invalid = golden.clone();
        invalid[at] = 0xff;
        assert_eq!(
            Launch::decode(&invalid).err(),
            Some(Fail::Invalid),
            "utf8 at {at}"
        );
    }

    assert_eq!(
        Launch::decode(&replace_text(&golden, 42, 50, 2, "")).err(),
        Some(Fail::Invalid),
        "empty port name"
    );

    let duplicate = replace_text(&golden, 84, 92, 3, "in");
    assert_eq!(
        Launch::decode(&duplicate).err(),
        Some(Fail::Invalid),
        "duplicate port name"
    );
    let distinct = replace_text(&golden, 84, 92, 3, "xy");
    let decoded = Launch::decode(&distinct).unwrap();
    assert_eq!(decoded.ports[1].name, "xy");
    assert_eq!(decoded.ports[1].endpoint.id, 2);
}

#[test]
fn launch_decoder_validates_endpoints_and_limits() {
    let (_, golden) = launch_fixture();
    assert_eq!(
        Launch::decode(&with_word(&golden, 52, 0)).err(),
        Some(Fail::Invalid),
        "endpoint id"
    );
    assert_eq!(
        Launch::decode(&with_word(&golden, 60, 0)).err(),
        Some(Fail::Invalid),
        "zero capacity"
    );
    assert_eq!(
        Launch::decode(&with_word(&golden, 60, MAX_CAPACITY as u64 + 1)).err(),
        Some(Fail::Invalid),
        "oversize capacity"
    );
    assert_eq!(
        Launch::decode(&with_word(&golden, 68, 0)).err(),
        Some(Fail::Invalid),
        "seed"
    );
    for code in [0, 3, 256] {
        assert_eq!(
            Launch::decode(&with_word(&golden, 76, code)).err(),
            Some(Fail::Invalid),
            "direction {code}"
        );
    }

    assert_eq!(
        Launch::decode(&launch_header(MAX_ARGS as u64 + 1, 0)).err(),
        Some(Fail::Full)
    );
    assert_eq!(
        Launch::decode(&launch_header(0, MAX_PORTS as u64 + 1)).err(),
        Some(Fail::Full)
    );
    let oversize = vec![0u8; MAX_SIZE + 1];
    assert_eq!(Launch::decode(&oversize).err(), Some(Fail::Full));

    let too_many_args = Launch {
        args: vec![String::new(); MAX_ARGS + 1],
        ports: Vec::new(),
    };
    assert_eq!(too_many_args.encode(), Err(Fail::Full));

    let mut ports = Vec::new();
    for at in 0..=MAX_PORTS {
        ports.push(Binding {
            name: format!("p{at}"),
            endpoint: Endpoint {
                id: at as u64 + 1,
                capacity: 1,
                seed: token(1),
                direction: Direction::Read,
            },
        });
    }
    assert_eq!(
        Launch {
            args: Vec::new(),
            ports,
        }
        .encode(),
        Err(Fail::Full)
    );

    let oversize_args = vec!["a".repeat(1100); MAX_ARGS];
    assert_eq!(
        Launch {
            args: oversize_args,
            ports: Vec::new(),
        }
        .encode(),
        Err(Fail::Full)
    );

    let empty = Launch {
        args: vec![String::new()],
        ports: Vec::new(),
    };
    let decoded = Launch::decode(&empty.encode().unwrap()).unwrap();
    assert_eq!(decoded.args, empty.args);
    assert!(decoded.ports.is_empty());
}

#[test]
fn launch_encoder_accepts_endpoint_fields_the_decoder_rejects() {
    // Preserved asymmetry: encoding does not validate endpoint identity or capacity.
    for (id, capacity, seed) in [
        (0u64, 1usize, 1u64),
        (1, 0, 1),
        (1, MAX_CAPACITY + 1, 1),
        (1, 1, 0),
    ] {
        let launch = Launch {
            args: Vec::new(),
            ports: vec![Binding {
                name: "p".into(),
                endpoint: Endpoint {
                    id,
                    capacity,
                    seed: token(seed),
                    direction: Direction::Read,
                },
            }],
        };
        let bytes = launch
            .encode()
            .expect("encoder does not validate endpoint fields");
        assert_eq!(
            Launch::decode(&bytes).err(),
            Some(Fail::Invalid),
            "{id}/{capacity}/{seed}"
        );
    }

    let empty_name = Launch {
        args: Vec::new(),
        ports: vec![Binding {
            name: String::new(),
            endpoint: Endpoint {
                id: 1,
                capacity: 1,
                seed: token(1),
                direction: Direction::Read,
            },
        }],
    };
    assert_eq!(empty_name.encode(), Err(Fail::Invalid));

    let duplicate_name = Launch {
        args: Vec::new(),
        ports: vec![
            Binding {
                name: "p".into(),
                endpoint: Endpoint {
                    id: 1,
                    capacity: 1,
                    seed: token(1),
                    direction: Direction::Read,
                },
            },
            Binding {
                name: "p".into(),
                endpoint: Endpoint {
                    id: 2,
                    capacity: 1,
                    seed: token(1),
                    direction: Direction::Read,
                },
            },
        ],
    };
    assert_eq!(duplicate_name.encode(), Err(Fail::Invalid));
}

#[test]
fn catalogue_golden_wire_is_stable_in_both_directions() {
    let (catalogue, golden) = catalogue_fixture();
    assert_eq!(catalogue.encode().unwrap(), golden);

    let decoded = Catalogue::decode(&golden).unwrap();
    assert_eq!(decoded.images.len(), catalogue.images.len());
    for (got, want) in decoded.images.iter().zip(&catalogue.images) {
        assert_eq!(got.name, want.name);
        assert_eq!(got.seed, want.seed);
        assert_eq!(got.mark, want.mark);
        assert_eq!(got.length, want.length);
        assert_eq!(got.ports, want.ports);
    }
}

#[test]
fn catalogue_decoder_rejects_truncation_trailing_and_names() {
    let (_, golden) = catalogue_fixture();
    for length in 0..golden.len() {
        assert_eq!(
            Catalogue::decode(&golden[..length]).err(),
            Some(Fail::Invalid),
            "{length}"
        );
    }

    let mut trailing = golden.clone();
    trailing.push(0);
    assert_eq!(Catalogue::decode(&trailing).err(), Some(Fail::Invalid));

    let mut magic = golden.clone();
    magic[0] ^= 1;
    assert_eq!(Catalogue::decode(&magic).err(), Some(Fail::Invalid));

    let mut version = golden.clone();
    version[8] = 4;
    assert_eq!(Catalogue::decode(&version).err(), Some(Fail::Invalid));

    for at in [32, 76] {
        let mut invalid = golden.clone();
        invalid[at] = 0xff;
        assert_eq!(
            Catalogue::decode(&invalid).err(),
            Some(Fail::Invalid),
            "utf8 at {at}"
        );
    }

    assert_eq!(
        Catalogue::decode(&replace_text(&golden, 24, 32, 4, "")).err(),
        Some(Fail::Invalid),
        "empty image name"
    );

    let duplicate_image = replace_text(&golden, 91, 99, 3, "emit");
    assert_eq!(
        Catalogue::decode(&duplicate_image).err(),
        Some(Fail::Invalid),
        "duplicate image name"
    );
    let distinct_image = replace_text(&golden, 91, 99, 3, "echo");
    let decoded = Catalogue::decode(&distinct_image).unwrap();
    assert_eq!(decoded.images[1].name, "echo");
    assert_eq!(decoded.images[1].seed, token(0x22));

    assert_eq!(
        Catalogue::decode(&replace_text(&golden, 68, 76, 7, "")).err(),
        Some(Fail::Invalid),
        "empty port name"
    );

    let duplicate_port = replace_text(&golden, 156, 164, 4, "source");
    assert_eq!(
        Catalogue::decode(&duplicate_port).err(),
        Some(Fail::Invalid),
        "duplicate port name"
    );
    let distinct_port = replace_text(&golden, 156, 164, 4, "target");
    let decoded = Catalogue::decode(&distinct_port).unwrap();
    assert_eq!(decoded.images[1].ports[1].0, "target");
    assert_eq!(decoded.images[1].ports[1].1, Direction::Write);

    for at in [83, 148] {
        for code in [0, 3, 256] {
            assert_eq!(
                Catalogue::decode(&with_word(&golden, at, code)).err(),
                Some(Fail::Invalid),
                "direction {code} at {at}"
            );
        }
    }
}

#[test]
fn catalogue_decoder_limits_and_encoder_asymmetries() {
    assert_eq!(
        Catalogue::decode(&catalogue_header(33)).err(),
        Some(Fail::Full)
    );
    let oversize = vec![0u8; MAX_SIZE + 1];
    assert_eq!(Catalogue::decode(&oversize).err(), Some(Fail::Full));

    let too_many_images = Catalogue {
        images: (0..=32).map(|at| image(&format!("i{at}"), 1, 1, Vec::new())).collect(),
    };
    assert_eq!(too_many_images.encode(), Err(Fail::Full));

    let mut oversize = Vec::new();
    for _ in 0..32 {
        oversize.push(image(&"a".repeat(2100), 1, 1, Vec::new()));
    }
    assert_eq!(Catalogue { images: oversize }.encode(), Err(Fail::Full));

    // Preserved asymmetries: the encoder does not enforce image name uniqueness
    // or non-emptiness, seed/length validity, or port name/count validity.
    for catalogue in [
        Catalogue {
            images: vec![image("", 1, 1, Vec::new())],
        },
        Catalogue {
            images: vec![image("x", 1, 1, Vec::new()), image("x", 1, 1, Vec::new())],
        },
        Catalogue {
            images: vec![image("x", 0, 1, Vec::new())],
        },
        Catalogue {
            images: vec![image("x", 1, 0, Vec::new())],
        },
        Catalogue {
            images: vec![image("x", 1, 1, vec![(String::new(), Direction::Read)])],
        },
        Catalogue {
            images: vec![image(
                "x",
                1,
                1,
                vec![
                    ("p".into(), Direction::Read),
                    ("p".into(), Direction::Write),
                ],
            )],
        },
    ] {
        let bytes = catalogue
            .encode()
            .expect("encoder does not validate catalogue semantics");
        assert_eq!(
            Catalogue::decode(&bytes).err(),
            Some(Fail::Invalid),
            "encoded catalogue must be rejected on decode"
        );
    }

    let mut ports = Vec::new();
    for at in 0..=MAX_PORTS {
        ports.push((format!("p{at}"), Direction::Read));
    }
    let bytes = Catalogue {
        images: vec![image("x", 1, 1, ports)],
    }
    .encode()
    .expect("encoder does not validate per-image port count");
    assert_eq!(Catalogue::decode(&bytes).err(), Some(Fail::Invalid));
}

#[test]
fn caps_marks_and_exact_max_size_payloads_are_frozen() {
    assert_eq!(MAX_ARGS, 64);
    assert_eq!(MAX_PORTS, 64);
    assert_eq!(MAX_SIZE, 65536);
    assert_eq!(shell_api::MARK, Mark::of("shell-launch"));
    assert_eq!(shell_api::CATALOGUE, Mark::of("shell-catalogue"));

    let launch = Launch {
        args: (0..MAX_ARGS).map(|at| format!("a{at}")).collect(),
        ports: Vec::new(),
    };
    let decoded = Launch::decode(&launch.encode().unwrap()).unwrap();
    assert_eq!(decoded.args.len(), MAX_ARGS);

    let mut ports = Vec::new();
    for at in 0..MAX_PORTS {
        ports.push(Binding {
            name: format!("p{at}"),
            endpoint: Endpoint {
                id: at as u64 + 1,
                capacity: 1,
                seed: token(1),
                direction: if at % 2 == 0 {
                    Direction::Read
                } else {
                    Direction::Write
                },
            },
        });
    }
    let launch = Launch {
        args: Vec::new(),
        ports,
    };
    let decoded = Launch::decode(&launch.encode().unwrap()).unwrap();
    assert_eq!(decoded.ports.len(), MAX_PORTS);
    assert_eq!(
        decoded.ports[MAX_PORTS - 1].name,
        format!("p{}", MAX_PORTS - 1)
    );

    let images: Vec<Image> = (0..32)
        .map(|at| image(&format!("i{at}"), 1, 1, Vec::new()))
        .collect();
    let decoded = Catalogue::decode(&Catalogue { images }.encode().unwrap()).unwrap();
    assert_eq!(decoded.images.len(), 32);
    assert_eq!(decoded.images[31].name, "i31");

    let launch = max_size_launch(0);
    let exact = launch.encode().unwrap();
    assert_eq!(exact.len(), MAX_SIZE);
    assert_eq!(Launch::decode(&exact).unwrap().args.len(), MAX_ARGS);
    assert_eq!(max_size_launch(1).encode(), Err(Fail::Full));
    let mut oversize = exact;
    oversize.push(0);
    assert_eq!(Launch::decode(&oversize).err(), Some(Fail::Full));

    let catalogue = max_size_catalogue(0);
    let exact = catalogue.encode().unwrap();
    assert_eq!(exact.len(), MAX_SIZE);
    assert_eq!(Catalogue::decode(&exact).unwrap().images.len(), 32);
    assert_eq!(max_size_catalogue(1).encode(), Err(Fail::Full));
    let mut oversize = exact;
    oversize.push(0);
    assert_eq!(Catalogue::decode(&oversize).err(), Some(Fail::Full));
}
