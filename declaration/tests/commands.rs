use env::{Mark, Permission, ProgramKind};
use pipe_api::Direction;
use shell_declaration::unit::{
    Construction, ConstructionArguments, ConstructionIdentity, ConstructionOwner,
    ConstructionRange, ConstructionSelector, Ready,
};
use shell_declaration::{BINDINGS, COMMANDS, PROGRAM};

const NAMES: [&str; 6] = ["cat", "emit", "upper", "fail", "spin", "workers"];

#[test]
fn command_order_and_ports_are_frozen() {
    let expected_ports: &[&[(&str, Direction)]] = &[
        &[("source", Direction::Read), ("copy", Direction::Write)],
        &[("records", Direction::Write)],
        &[("source", Direction::Read), ("result", Direction::Write)],
        &[],
        &[],
        &[("ticks", Direction::Write)],
    ];

    assert_eq!(COMMANDS.len(), NAMES.len());
    for (at, name) in NAMES.iter().enumerate() {
        assert_eq!(COMMANDS[at].name, *name);
        assert_eq!(COMMANDS[at].ports, expected_ports[at]);
    }
}

#[test]
fn bindings_align_with_commands_and_fetch_initrd() {
    assert_eq!(BINDINGS.len(), COMMANDS.len());
    assert_eq!(PROGRAM.supply.bindings.len(), BINDINGS.len());

    for (at, command) in COMMANDS.iter().enumerate() {
        let binding = BINDINGS[at];
        assert_eq!(binding, command.binding);
        assert_eq!(PROGRAM.supply.bindings[at], binding);
        assert_eq!(binding.source.name(), "system/initrd");
        assert_eq!(binding.resource.name(), command.name);
        assert_eq!(binding.slot, Mark::of(&format!("shell-{}", command.name)));
        assert_eq!(binding.permission, Permission::FETCH);
    }
}

#[test]
fn program_is_shell_image_with_image_construction() {
    assert_eq!(PROGRAM.name, "shell");
    assert_eq!(PROGRAM.loader.image, "shell");
    assert_eq!(PROGRAM.loader.space, ProgramKind::User);
    assert!(PROGRAM.loader.args.is_empty());
    assert_eq!(PROGRAM.loader.stack, 0);
    assert!(PROGRAM.operator.namespace.is_empty());
    assert!(PROGRAM.identity.aliases.is_empty());
    assert!(PROGRAM.identity.grants.is_empty());

    assert_eq!(PROGRAM.control.ready, Ready::OnSpawn);
    assert!(PROGRAM.control.after.is_none());
    assert!(PROGRAM.control.ending.is_none());
    assert!(PROGRAM.control.supervision.is_none());
    assert!(PROGRAM.control.retirement.is_empty());
    assert_eq!(PROGRAM.control.construction.len(), 1);
    assert!(PROGRAM.control.construction[0] == Construction::image());

    let image = Construction::image();
    assert!(image.selector == ConstructionSelector::Image);
    assert!(image.owner == ConstructionOwner::Caller);
    assert!(image.identity == ConstructionIdentity::Caller);
    assert!(image.arguments == (ConstructionArguments { min: 0, max: 64 }));
    assert!(
        image.stack
            == (ConstructionRange {
                min: 0,
                max: 1 << 20,
            })
    );
    assert_eq!(image.max_image, system_api::loader::MAX_IMAGE);
    assert!(image.valid());
}
