use amun_consensus_math::Fixed;
use amun_state_machine::{ConstitutionalState, Event, EventType, TransitionEngine};

fn main() {
    println!("Testing Constitutional State Machine\n");

    // Create initial state
    let mut state = ConstitutionalState::new();
    state.add_account(1, Fixed::from_int(1000));
    state.add_account(2, Fixed::from_int(500));

    println!("Initial state:");
    println!("  Account 1: {} amun", state.get_account(1).unwrap().balance.to_float());
    println!("  Account 2: {} amun", state.get_account(2).unwrap().balance.to_float());
    println!("  Total supply: {} amun", state.total_supply.to_float());
    println!("  State hash: {:02x?}", &state.hash()[..4]);

    // Transfer 100 from account 1 to account 2
    let event = Event::new(EventType::Transfer, 1, 2, Fixed::from_int(100).raw(), 0);
    let result = TransitionEngine::apply(&mut state, &event);

    if result.success {
        println!("\nAfter transfer 100 from 1 -> 2:");
        println!("  Account 1: {} amun", state.get_account(1).unwrap().balance.to_float());
        println!("  Account 2: {} amun", state.get_account(2).unwrap().balance.to_float());
        println!("  State hash: {:02x?}", &state.hash()[..4]);
    } else {
        println!("\nTransfer failed: {:?}", result.error_message);
    }

    // Try insufficient balance
    let event2 = Event::new(EventType::Transfer, 1, 2, Fixed::from_int(10000).raw(), 1);
    let result2 = TransitionEngine::apply(&mut state, &event2);

    if !result2.success {
        println!("\nExpected failure: {:?}", result2.error_message);
    }

    println!("\nFinal state hash: {:02x?}", &state.hash()[..4]);
    println!("\n✅ State machine test completed");
}
