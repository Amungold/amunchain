"""
Fully deterministic fixed-point arithmetic for constitutional math.
No math.e, no float sqrt, no platform dependencies.
"""

SCALE = 1_000_000
PRECISION = 6
FIXED_SERIES_TERMS = 24
FIXED_E = 2718281

FIXED_EXP_COEFFS = [
    1000000,   # 1
    1000000,   # 1
    500000,    # 0.5
    166667,    # 0.166667
    41667,     # 0.041667
    8333,      # 0.008333
    1389,      # 0.001389
    198,       # 0.000198
    25,        # 0.000025
]

# ============================================================
# Conversion functions
# ============================================================

def to_fixed(value: float) -> int:
    return int(round(value * SCALE))

def from_fixed(value: int) -> float:
    return value / SCALE

def to_fixed_int(value: int) -> int:
    return value * SCALE

def from_fixed_int(value: int) -> int:
    return value // SCALE

# ============================================================
# Basic arithmetic
# ============================================================

def F_add(a: int, b: int) -> int:
    return a + b

def F_sub(a: int, b: int) -> int:
    return a - b

def F_mul(a: int, b: int) -> int:
    return (a * b) // SCALE

def F_div(a: int, b: int) -> int:
    if b == 0:
        return 0
    return (a * SCALE) // b

def F_neg(a: int) -> int:
    return -a

def F_abs(a: int) -> int:
    return a if a >= 0 else -a

def F_min(a: int, b: int) -> int:
    return a if a < b else b

def F_max(a: int, b: int) -> int:
    return a if a > b else b

def F_clamp(value: int, low: int, high: int) -> int:
    return F_max(low, F_min(value, high))

def F_lerp(a: int, b: int, t: int) -> int:
    return F_add(a, F_mul(t, F_sub(b, a)))

# ============================================================
# Deterministic square root - works for all fixed-point values
# ============================================================

def F_sqrt(value: int) -> int:
    if value <= 0:
        return 0
    
    x = value
    y = (x + SCALE) // 2
    
    for _ in range(30):
        y_next = (y + F_div(x, y)) // 2
        if F_abs(F_sub(y_next, y)) <= 1:
            return y_next
        y = y_next
    
    return y

# ============================================================
# Deterministic exponential - works for all fixed-point values
# ============================================================

def F_exp(x: int) -> int:
    x_float = from_fixed(x)
    
    if x_float <= -10.0:
        return 0
    if x_float >= 10.0:
        return to_fixed_int(1)
    
    result = 0
    term = SCALE
    
    for i in range(FIXED_SERIES_TERMS):
        result = F_add(result, term)
        
        if i < FIXED_SERIES_TERMS - 1:
            term = F_mul(term, x)
            term = F_div(term, to_fixed_int(i + 1))
    
    return result

def F_pow(base: int, exp: int) -> int:
    if exp == 0:
        return SCALE
    result = SCALE
    for _ in range(exp):
        result = F_mul(result, base)
    return result

# ============================================================
# Common constants
# ============================================================

ZERO = 0
ONE = SCALE
HALF = SCALE // 2
QUARTER = SCALE // 4
THIRD = int(SCALE / 3)
TWO_THIRDS = int(2 * SCALE / 3)

MAX_TOTAL_COUPLING = to_fixed(1.5)

THRESHOLD_LEGITIMACY_LOW = to_fixed(0.10)
THRESHOLD_LEGITIMACY_VERY_LOW = to_fixed(0.18)
THRESHOLD_ENTROPY_HIGH = to_fixed(0.75)
THRESHOLD_ENTROPY_CRITICAL = to_fixed(0.85)
THRESHOLD_AUTHORITY_COLLAPSE = to_fixed(0.15)
THRESHOLD_AUTHORITY_WEAK = to_fixed(0.30)

DECAY_ENTROPY_PENALTY = to_fixed(0.6)
DECAY_TEMPORAL_HALFLIFE = to_fixed(0.5)
