"""
Fixed-point math kernel v2 - Matches Rust implementation exactly.
NO FLOATING POINT in core operations.
"""

SCALE = 1_000_000
MAX_FIXED = 10_000_000_000 * SCALE

def to_fixed(value: float) -> int:
    return int(round(value * SCALE))

def from_fixed(value: int) -> float:
    return value / SCALE

def saturating_add(a: int, b: int) -> int:
    result = a + b
    if result > MAX_FIXED:
        return MAX_FIXED
    if result < -MAX_FIXED:
        return -MAX_FIXED
    return result

def saturating_mul(a: int, b: int) -> int:
    product = a * b
    scaled = product // SCALE
    if scaled > MAX_FIXED:
        return MAX_FIXED
    if scaled < -MAX_FIXED:
        return -MAX_FIXED
    return scaled

def F_sqrt(x: int) -> int:
    if x <= 0:
        return 0
    
    y = (x + SCALE) // 2
    
    for _ in range(30):
        x_div_y = (x * SCALE) // y
        y_next = (y + x_div_y) // 2
        if abs(y_next - y) <= 1:
            return y_next
        y = y_next
    return y

def F_exp(x: int) -> int:
    if x <= -10 * SCALE:
        return 0
    if x >= 10 * SCALE:
        return MAX_FIXED
    
    a = x // SCALE
    r = x % SCALE
    
    result = 0
    term = SCALE
    fact = 1
    
    for n in range(24):
        result = saturating_add(result, term)
        if n < 23:
            term = saturating_mul(term, r)
            fact *= (n + 1)
            term = term // fact
    
    FIXED_E = 2718281
    
    if a > 0:
        for _ in range(a):
            result = saturating_mul(result, FIXED_E)
    elif a < 0:
        for _ in range(-a):
            result = result // FIXED_E
    
    if result > MAX_FIXED:
        return MAX_FIXED
    if result < -MAX_FIXED:
        return -MAX_FIXED
    return result
