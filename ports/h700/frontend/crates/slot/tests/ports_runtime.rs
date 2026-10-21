#[test]
fn installed_runtime_can_request_each_port_on_each_device() {
    for device in ["rgsp", "rg35xxsp", "g350"] {
        std::env::set_var("SLOT_DEVICE", device);
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        for dir in ["System", "Games/PORTS", "Config"] {
            std::fs::create_dir_all(root.join(dir)).unwrap();
        }
        let runner = root.join(if device == "g350" {
            "System/slot-port-runner.sh"
        } else {
            "System/slot-h700-port-runner.sh"
        });
        std::fs::write(&runner, "#!/bin/sh\n").unwrap();
        let descriptor = root.join("Games/PORTS/game.port");
        for id in ["portmaster", "script:Grand Theft Auto San Andreas.sh", "script:Need for Speed Underground 2.sh"] {
            std::fs::write(&descriptor, id).unwrap();
            slot::ports::request(root, &descriptor).unwrap();
            assert_eq!(std::fs::read_to_string(root.join("Config/port.request")).unwrap(), id);
        }
        std::fs::remove_file(runner).unwrap();
        assert_eq!(slot::ports::request(root, &descriptor).unwrap_err().kind(), std::io::ErrorKind::NotFound);
    }
    std::env::remove_var("SLOT_DEVICE");
}
