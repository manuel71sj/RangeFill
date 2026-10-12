use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Settings {
    /// 최대 주유 시 주행가능거리 (km)
    pub a: f64,
    /// 연비 (km/L)
    pub c: f64,
    /// 주유단가 (원/L)
    pub d: f64,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Quote {
    pub liters: f64,
    pub cost: f64,
}

impl Settings {
    pub fn validate(self) -> Result<Self, String> {
        if [self.a, self.c, self.d]
            .iter()
            .all(|v| v.is_finite() && *v > 0.0)
        {
            Ok(self)
        } else {
            Err("최대 거리, 연비, 주유단가는 0보다 큰 유한한 숫자여야 합니다.".into())
        }
    }
}

/// `b`는 현재 남은 주행가능거리(km). 주유량은 소수 둘째 자리, 금액은 원 단위로 반올림한다.
pub fn quote(settings: Settings, b: f64) -> Result<Quote, String> {
    let Settings { a, c, d } = settings.validate()?;
    if !b.is_finite() || b < 0.0 {
        return Err("남은 거리는 0 이상의 유한한 숫자여야 합니다.".into());
    }
    let liters = (a - b).max(0.0) / c;
    let result = Quote {
        liters: (liters * 100.0).round() / 100.0,
        cost: (liters * d).round(),
    };
    if result.liters.is_finite() && result.cost.is_finite() {
        Ok(result)
    } else {
        Err("계산 결과가 너무 큽니다.".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings(a: f64, c: f64, d: f64) -> Settings {
        Settings { a, c, d }
    }

    #[test]
    fn rounds_liters_to_two_decimals_and_cost_to_won() {
        let s = settings(600.0, 15.0, 1700.0);
        assert_eq!(
            quote(s, 150.0),
            Ok(Quote {
                liters: 30.0,
                cost: 51000.0
            })
        );
        assert_eq!(
            quote(s, 100.0),
            Ok(Quote {
                liters: 33.33,
                cost: 56667.0
            })
        );
    }

    #[test]
    fn remaining_at_or_above_maximum_is_zero() {
        let s = settings(600.0, 15.0, 1700.0);
        assert_eq!(
            quote(s, 600.0),
            Ok(Quote {
                liters: 0.0,
                cost: 0.0
            })
        );
        assert_eq!(
            quote(s, 700.0),
            Ok(Quote {
                liters: 0.0,
                cost: 0.0
            })
        );
    }

    #[test]
    fn rejects_invalid_values() {
        assert!(quote(settings(0.0, 15.0, 1700.0), 10.0).is_err());
        assert!(quote(settings(600.0, f64::NAN, 1700.0), 10.0).is_err());
        assert!(quote(settings(600.0, 15.0, f64::INFINITY), 10.0).is_err());
        assert!(quote(settings(600.0, 15.0, 1700.0), -1.0).is_err());
        assert!(quote(settings(600.0, 15.0, 1700.0), f64::NAN).is_err());
    }
}
