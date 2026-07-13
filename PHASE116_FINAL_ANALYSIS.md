# Phase 116 — Constitutional Sensitivity Analysis: Final Results

## Overview

Phase 116 evaluates how AmunChain's constitutional parameters respond to
changes in three factors: f₂ (secondary damping), f₃ (primary driver), and
two interventions — Recognition↑ and Treaties↑.

All three test suites pass: 3/3 tests, 0 failures.

## Results

### A. Main Effects (Phase 116A)

| Condition | Ē (Efficiency) | C_R (Stability) | D_L (Divergence) |
|-----------|----------------|-----------------|------------------|
| Baseline | 0.2862 | 1.0000 | 0.1063 |
| Max f₂ | 0.2666 | 1.0000 | 0.1034 |
| Max f₃ | 0.3771 | 1.0000 | 0.1252 |

- f₂ effect: -6.8% (negative, weak)
- f₃ effect: +31.8% (positive, dominant)
- C_R remains 1.0000 in all conditions — structural stability preserved

### B. Interaction Effects (Phase 116B)

| f₂ | f₃ | Ē |
|----|----|-----|
| Low | Low | 0.2742 |
| High | Low | 0.2539 |
| Low | High | 0.4089 |
| High | High | 0.3862 |

- f₃ is the primary driver at all f₂ levels
- f₂ acts as a damping factor, not an independent driver
- Best configuration: Low f₂ + High f₃ (Ē = 0.4089)

### C. Engineering Map (Phase 116C)

| Intervention | ΔĒ | Efficiency Ratio |
|-------------|-----|-----------------|
| Recognition↑ | +0.0169 | 0.338 |
| Treaties↑ | +0.0751 | 0.150 |
| Combined↑ | +0.0874 | 0.159 |

- Treaties↑ is 4.4× more effective than Recognition↑
- Combined effect shows partial saturation (-5% from linear sum)
- No negative side effects observed

## Conclusions

1. **Dominant driver**: f₃ controls system behavior with +31.8% effect
2. **Damping factor**: f₂ reduces efficiency by -6.8% without affecting stability
3. **Best intervention**: Treaties↑ delivers 4.4× more benefit than Recognition↑
4. **System stability**: C_R = 1.0000 across all conditions — constitutionally sound

## Significance for AmunChain

If the model represents constitutional network engineering, the results
indicate that institutional linkages (Treaties) yield the highest gains,
while recognition alone has limited impact. Removing f₂-related inhibitors
is more important than attempting to optimize them. The constitutional
core remains stable under all tested perturbations.
