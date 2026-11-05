use shell_declaration::{BINDINGS, BUILD, PROGRAM};
use unit_contract::{document, record};

fn declaration() -> document::Unit<String> {
    toml_edit::de::from_str(include_str!("../../server/entry/unit.toml")).unwrap()
}

#[test]
fn runtime_declaration_keeps_shell_permissions_and_cargo_identity() {
    let declared = declaration();
    declared.validate().unwrap();
    assert_eq!(declared.id, PROGRAM.name);
    assert_eq!(declared.loader.image, BUILD.bin);
    assert_eq!(declared.loader.space, document::Space::User);
    assert_eq!(declared.loader.args.len(), PROGRAM.loader.args.len());
    assert_eq!(declared.loader.stack, PROGRAM.loader.stack as u64);
    assert!(declared.operator.namespace.is_empty());
    assert!(declared.identity.aliases.is_empty());
    assert!(declared.identity.grants.is_empty());
    assert_eq!(declared.control.creation, document::Creation::Requested);
    assert_eq!(declared.control.requires, ["pipe", "terminal"]);
    assert!(declared.control.after.is_empty());
    assert_eq!(declared.control.ready, document::Ready::Spawn);
    assert!(declared.control.supervision.is_none());
    assert!(declared.control.retirement.is_empty());
    assert_eq!(
        declared.control.construction.len(),
        PROGRAM.control.construction.len()
    );
    let expected = PROGRAM.control.construction[0];
    let policy = &declared.control.construction[0];
    assert!(policy.unit.is_none());
    assert_eq!(policy.owner, document::Owner::Caller);
    assert_eq!(policy.identity, document::IdentitySelection::Caller);
    assert_eq!(policy.arguments.min, expected.arguments.min);
    assert_eq!(policy.arguments.max, expected.arguments.max);
    assert_eq!(policy.stack.min, expected.stack.min);
    assert_eq!(policy.stack.max, expected.stack.max);
    assert_eq!(policy.max_image, expected.max_image as u64);
}

#[test]
fn resources_keep_slots_and_permissions_while_using_real_cargo_bins() {
    let declared = declaration();
    assert_eq!(declared.supply.bindings.len(), BINDINGS.len());
    for (actual, expected) in declared.supply.bindings.iter().zip(BINDINGS) {
        let image = match expected.resource.name() {
            "cat" => "prog-cat",
            other => other,
        };
        assert_eq!(actual.source, expected.source.name());
        assert_eq!(actual.resource, image);
        assert_eq!(actual.slot, expected.slot.get());
        assert_eq!(actual.permission, expected.permission.bits());
    }
    let bytes = record::encode(&declared).unwrap();
    let decoded = record::decode(&bytes).unwrap();
    assert_eq!(
        decoded.supply.bindings[1].slot,
        env::Mark::of("shell-emit").get()
    );
    assert_eq!(
        decoded.supply.bindings[3].slot,
        env::Mark::of("shell-fail").get()
    );
}
