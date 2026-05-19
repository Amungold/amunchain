# Network Adversary Law v1.0

## Article I: Adversary Model

The consensus protocol operates under the BYZANTINE ADVERSARY model.

### Adversary Capabilities
- Control up to f validators (f < n/3 by stake)
- Delay messages arbitrarily before GST
- Reorder messages arbitrarily
- Drop messages selectively (but not all)
- Partition the network temporarily
- Eclipse a minority of validators
- Rushing: see honest messages before sending own

### Adversary Limitations
- Cannot forge signatures (EUF-CMA assumption)
- Cannot find Blake3 collisions
- Cannot corrupt more than f validators simultaneously
- Cannot delay messages indefinitely after GST
- Cannot corrupt validators retroactively (past votes are safe)

## Article II: Adaptive Corruption Model

The adversary may corrupt validators DURING protocol execution.

### Corruption Latency
There is a delay of corruption_latency_rounds between the
adversary deciding to corrupt a validator and the corruption
taking effect. During this window, the validator is still honest.

### Slashability Window
Once slashed, a validator's stake is frozen for
slashability_window_epochs epochs. The stake cannot be
withdrawn or reused during this period.

### Unbonding Delay
Validators cannot withdraw stake immediately. There is an
unbonding_delay_epochs delay between requesting exit
and stake becoming available.

## Article III: Synchrony Model

The protocol uses the PARTIAL SYNCHRONY model:
- Before GST: adversary controls message delivery
- After GST: all messages delivered within delta rounds
- GST is unknown to validators

## Article IV: Message Delivery Semantics

Messages may be delayed, duplicated, reordered, or dropped.
Messages must be eventually delivered after GST.
Messages must be detectably corrupted via checksums.
Messages must be idempotently processable.

## Article V: Anti-Replay

All consensus messages carry:
- Chain ID (cross-chain replay prevention)
- Epoch number (cross-epoch replay prevention)
- Round number (cross-round replay prevention)
- Protocol version (cross-version replay prevention)
- Constitution hash (cross-fork replay prevention)

## Article VI: Byzantine Detection

The following are DETECTABLE Byzantine faults:
- Equivocation: two different votes in the same round
- Invalid proposal: block fails execution validation
- Lock violation: vote contradicts known lock
- Timeout fraud: falsified timeout certificate

All detected faults produce Level 0 Evidence for slashing.
