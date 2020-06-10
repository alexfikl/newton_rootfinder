use std::fmt;

/// Choice of the iterative algorithm for the resolution
///
/// All of them are Newton based methods : (Newton or quasi-Newton)
///
/// All Newton-based iterative methods have a local convergence.
/// They also assume that the jacobian is invertible at the root (simple root)
///
/// ## Newton-Raphson
/// The classical Newton method \[1995\]
///
/// Requires a full jacobian evaluation at each iteration step
///
/// ### Quasi-Newton Methods
///
/// Quasi Newton methods are used when the computation of the jacobian is too computationnaly expensive.
///
/// Instead of using the jacobian, there are using a approximation of this matrix (or its inverse).
/// In most of the case, a computation of the true jacobian is still required for initialization purpose.
///
/// ## Reference
///
/// ### Tjalling J. Ypma (1995)
///
/// Historical development of the Newton–Raphson method,
///
/// SIAM Review 37 (4), p 531–551, 1995.
///
/// doi:10.1137/1037125.
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResolutionMethod {
    NewtonRaphson,
    QuasiNewton(QuasiNewtonMethod),
}

impl fmt::Display for ResolutionMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut content = String::new();
        match self {
            ResolutionMethod::NewtonRaphson => content.push_str("Newton-Raphson"),
            ResolutionMethod::QuasiNewton(method) => content.push_str(&format!("Quasi Newton: {}", method.to_string())),
        };

        write!(f, "{}", content)
    }
}

/// This quasi-Newton methods either work on the jacobian or its inverse
///
///
/// ## Stationary Newton \[1967\]
/// A quasi Newton Method requiring the evaluation of the jacobian only at the first iteration step.
///
/// The jacobian of the first iteration is used for all the updates
///
/// The convergence rate is locally linear and controlled by the first error :
///
/// || x_{n+1} - x_sol || < || x_{n} - x_sol ||*|| x_{0} - x_sol ||
///
///
/// ## General form of others Quasi-Newton method considered
///
/// The general formula taken from \[1997\] is:
///
/// H_{i+1} = H_{i} - (H_{i}*y_{i}-s_{i})c_{i}^{T}/(c_{i}^{T}*y_{i}),
///
/// With, for the iteration i:
/// - H_{i} = -J_{i}^{-1}, the opposite of the inverse of the approximated jacobian
/// - s_{i} = x_{i+1} - x_{i}, the vector of the iterative update
/// - y_{i} = F_{x_{i+1}} - F_{x_{i}}, the vector of the residual update
/// - c_{i}, a vector that is chosen differently according to the method.
///
/// This method can also be applied, instead on the inverse of the jacobian, with the jacobian itself.
/// Householder's formula (also known as Sherman-Morrison's formula) yields:
///
/// J_{i+1} = J_{i} - (J_{i}*s_{i}-y_{i})*c_{i}^{T}*J_{i}/(c_{i}^{T}*J_{i}*s_{i})
///
///
///
/// ## Broyden methods
/// Two methods have been published by Broyden,
/// - The first method, knowned as "Broyden Good Method"
/// - The second method, knowned as "Broyden Bad Method"
///
/// For the different methods, c_{i} is taken as such:
/// - First method: c_{i} = H_{i}^{T} * s_{i}
/// - Second method: c_{i} = y_{i}
///
/// The update formulas are the following:
///
/// | Method   | c_{i} value        | Jacobian update                                                               | Inverse jacobian update |
/// |----------|--------------------|-------------------------------------------------------------------------------|-------------------------|
/// | First    | H_{i}^{T} * s_{i}  | J_{i+1} = J_{i} - (J_{i}*s_{i}-y_{i})*s_{i}^{T}/(s_{i}^{T}*s_{i})             |                         |
/// | Second   | y_{i}              | J_{i+1} = J_{i} - (J_{i}*s_{i}-y_{i})*y_{i}^{T}*J_{i}/(y_{i}^{T}*J_{i}*s_{i}) |                         |
///
///
/// ## Reference
///
/// ### Dennis, Jr., J. E. (1967)
///
/// A Stationary Newton Method for Nonlinear Functional Equations
///
/// SIAM Journal on Numerical Analysis, 4(2), p 222–232.
///
/// doi:10.1137/0704021
///
/// ### Spedicato, E. ; Huang, Z. (1996)
///
/// Numerical experience with Newton-like methods for nonlinear algebraic systems,
///
/// Computing, p 68-89.
///
/// doi:10.1007/BF02684472

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum QuasiNewtonMethod {
    StationaryNewton,
    BroydenGood,
    BroydenBad
}


impl fmt::Display for QuasiNewtonMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = match self {
            QuasiNewtonMethod::StationaryNewton => &"Stationary Newton",
            QuasiNewtonMethod::BroydenGood => &"Broyden Good Method",
            QuasiNewtonMethod::BroydenBad => &"Broyden Good Method",
        };

        write!(f, "{}", result)
    }
}
