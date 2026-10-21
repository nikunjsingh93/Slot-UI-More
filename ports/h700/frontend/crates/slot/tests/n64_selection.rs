mod common;
use common::{app_booting_at,CLOCK_IS_SET};
use slot::app::Phase;
use slot_input::{Action,Btn};
use slot_store::{Platform,SlotState};

#[test]
fn native_return_restores_exact_game_and_start_saves_only_that_games_emulator() {
    std::env::set_var("SLOT_N64_BACKEND","native-rice");
    let d=tempfile::tempdir().unwrap();let root=d.path();slot::root::ensure(root);
    for name in ["Wave Race","Diddy Kong Racing"] {
        std::fs::write(root.join(format!("Games/N64/{name}.z64")),[0;4096]).unwrap();
    }
    std::fs::write(root.join("Games/GB/First.gb"),[0;4096]).unwrap();
    slot_store::write_slot_state(root,&SlotState{clock_set:true,..Default::default()}).unwrap();
    slot::ports::remember_shelf(root,"N64","Wave Race").unwrap();
    let (mut app,_)=app_booting_at(root,CLOCK_IS_SET);
    assert!(matches!(app.phase(),Phase::Shelf));
    assert_eq!(app.selected_stem(),Some("Wave Race"));
    assert!(!root.join("Config/return-to-shelf").exists());
    assert!(slot::ports::n64_native(root,"Wave Race"));
    app.apply(Action::GbaDown(Btn::Select));
    assert!(slot::ports::n64_native(root,"Wave Race"), "Select must not change the emulator");
    app.apply(Action::GbaDown(Btn::Start));
    assert!(!slot::ports::n64_native(root,"Wave Race"));
    assert!(slot::ports::n64_native(root,"Diddy Kong Racing"));
    slot::ports::remember_shelf(root,"N64","Wave Race").unwrap();
    let (mut app,_)=app_booting_at(root,CLOCK_IS_SET);
    assert_eq!(app.selected_stem(),Some("Wave Race"));
    assert!(!slot::ports::n64_native(root,"Wave Race"));
    app.apply(Action::GbaDown(Btn::Start));
    assert!(slot::ports::n64_native(root,"Wave Race"));
    assert!(slot_store::ini::value(root,"Config/n64-emulator.ini",Platform::N64.dir_name()).is_none());
    std::env::remove_var("SLOT_N64_BACKEND");
}
