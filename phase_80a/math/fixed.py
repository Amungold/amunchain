from .ops import (
    SCALE, to_fixed, from_fixed, to_fixed_int, from_fixed_int,
    F_add, F_sub, F_mul, F_div, F_neg, F_abs, F_sqrt, F_exp, F_pow,
    F_min, F_max, F_clamp, F_lerp,
    ZERO, ONE, HALF, QUARTER, THIRD, TWO_THIRDS,
    THRESHOLD_LEGITIMACY_LOW, THRESHOLD_LEGITIMACY_VERY_LOW,
    THRESHOLD_ENTROPY_HIGH, THRESHOLD_ENTROPY_CRITICAL
)

class FixedNumber:
    def __init__(self, value):
        if isinstance(value, int):
            if abs(value) > SCALE * 100:
                self.value = value
            else:
                self.value = to_fixed_int(value)
        elif isinstance(value, float):
            self.value = to_fixed(value)
        elif isinstance(value, FixedNumber):
            self.value = value.value
        else:
            self.value = to_fixed(float(value))
    
    @classmethod
    def from_raw(cls, raw_value: int) -> 'FixedNumber':
        instance = cls.__new__(cls)
        instance.value = raw_value
        return instance
    
    def to_float(self) -> float:
        return from_fixed(self.value)
    
    def to_int(self) -> int:
        return from_fixed_int(self.value)
    
    def __add__(self, other):
        return FixedNumber.from_raw(F_add(self.value, FixedNumber(other).value))
    
    def __sub__(self, other):
        return FixedNumber.from_raw(F_sub(self.value, FixedNumber(other).value))
    
    def __mul__(self, other):
        return FixedNumber.from_raw(F_mul(self.value, FixedNumber(other).value))
    
    def __truediv__(self, other):
        return FixedNumber.from_raw(F_div(self.value, FixedNumber(other).value))
    
    def __neg__(self):
        return FixedNumber.from_raw(F_neg(self.value))
    
    def __abs__(self):
        return FixedNumber.from_raw(F_abs(self.value))
    
    def __eq__(self, other):
        return self.value == FixedNumber(other).value
    
    def __lt__(self, other):
        return self.value < FixedNumber(other).value
    
    def __le__(self, other):
        return self.value <= FixedNumber(other).value
    
    def __gt__(self, other):
        return self.value > FixedNumber(other).value
    
    def __ge__(self, other):
        return self.value >= FixedNumber(other).value
    
    def __repr__(self):
        return f"FixedNumber({self.to_float():.6f})"
    
    def sqrt(self):
        return FixedNumber.from_raw(F_sqrt(self.value))
    
    def exp(self):
        return FixedNumber.from_raw(F_exp(self.value))
    
    def pow(self, exp: int):
        return FixedNumber.from_raw(F_pow(self.value, exp))
    
    def clamp(self, low, high):
        low_val = FixedNumber(low).value
        high_val = FixedNumber(high).value
        return FixedNumber.from_raw(F_clamp(self.value, low_val, high_val))
    
    def lerp(self, target, t):
        target_val = FixedNumber(target).value
        t_val = FixedNumber(t).value
        return FixedNumber.from_raw(F_lerp(self.value, target_val, t_val))
    
    @property
    def is_zero(self) -> bool:
        return self.value == 0
    
    @property
    def is_positive(self) -> bool:
        return self.value > 0
    
    @property
    def is_negative(self) -> bool:
        return self.value < 0

ZERO_F = FixedNumber.from_raw(ZERO)
ONE_F = FixedNumber.from_raw(ONE)
HALF_F = FixedNumber.from_raw(HALF)
QUARTER_F = FixedNumber.from_raw(QUARTER)
THIRD_F = FixedNumber.from_raw(THIRD)
TWO_THIRDS_F = FixedNumber.from_raw(TWO_THIRDS)

THRESHOLD_LEGITIMACY_LOW_F = FixedNumber.from_raw(THRESHOLD_LEGITIMACY_LOW)
THRESHOLD_LEGITIMACY_VERY_LOW_F = FixedNumber.from_raw(THRESHOLD_LEGITIMACY_VERY_LOW)
THRESHOLD_ENTROPY_HIGH_F = FixedNumber.from_raw(THRESHOLD_ENTROPY_HIGH)
THRESHOLD_ENTROPY_CRITICAL_F = FixedNumber.from_raw(THRESHOLD_ENTROPY_CRITICAL)
