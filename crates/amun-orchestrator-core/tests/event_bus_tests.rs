use amun_orchestrator_core::event::{EventBus, EventSeverity, OrchestratorEvent};

#[test]
fn test_publish_and_receive() {
    let bus = EventBus::new(16);
    let mut rx = bus.subscribe();
    bus.emit("test", OrchestratorEvent::BuildStarted { crate_count: 5 });
    let e = rx.try_recv().unwrap();
    assert_eq!(e.event, OrchestratorEvent::BuildStarted { crate_count: 5 });
    assert_eq!(e.source, "test");
}

#[test]
fn test_multiple_subscribers() {
    let bus = EventBus::new(16);
    let mut rx1 = bus.subscribe();
    let mut rx2 = bus.subscribe();
    bus.emit(
        "test",
        OrchestratorEvent::QuorumReached {
            validators: 4,
            total_power: 400,
        },
    );
    assert_eq!(rx1.try_recv().unwrap().event, rx2.try_recv().unwrap().event);
}

#[test]
fn test_event_severity() {
    assert_eq!(
        OrchestratorEvent::ValidatorCrashed {
            name: "v1".into(),
            error: "OOM".into(),
            crash_count: 1,
        }
        .severity(),
        EventSeverity::Error
    );
    assert_eq!(
        OrchestratorEvent::GenesisGenerated {
            path: "/tmp/g.json".into()
        }
        .severity(),
        EventSeverity::Audit
    );
}
