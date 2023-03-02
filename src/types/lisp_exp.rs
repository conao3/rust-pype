use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use super::lisp_atom::LispAtom;

pub type LispExpRef = Weak<RefCell<LispExp>>;
pub type LispExpRefStrong = Rc<RefCell<LispExp>>;

#[derive(Debug)]
pub enum LispExp {
    Atom(LispAtom),
    Cons { car: LispExpRef, cdr: LispExpRef },
}

impl std::fmt::Display for LispExp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LispExp::Atom(e) => write!(f, "{}", e),
            LispExp::Cons { .. } => {
                let mut lst: Vec<String> = Vec::new();

                for (car, cdr) in self.cons_iter_ptr() {
                    lst.push(format!("{}", car.borrow()));
                    match &*cdr.borrow() {
                        LispExp::Atom(LispAtom::Symbol(s)) if s == "nil" => {}
                        LispExp::Atom(_) => {
                            lst.push(".".to_string());
                            lst.push(format!("{}", cdr.borrow()));
                        }
                        LispExp::Cons { .. } => (),
                    }
                }

                write!(f, "({})", lst.join(" "))
            }
        }
    }
}

impl PartialEq for LispExp {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LispExp::Atom(e1), LispExp::Atom(e2)) => e1 == e2,
            (
                LispExp::Cons {
                    car: car1,
                    cdr: cdr1,
                },
                LispExp::Cons {
                    car: car2,
                    cdr: cdr2,
                },
            ) => {
                let car1_rc = car1.upgrade().unwrap();
                let car2_rc = car2.upgrade().unwrap();
                let cdr1_rc = cdr1.upgrade().unwrap();
                let cdr2_rc = cdr2.upgrade().unwrap();

                let car1_ = car1_rc.borrow();
                let car2_ = car2_rc.borrow();
                let cdr1_ = cdr1_rc.borrow();
                let cdr2_ = cdr2_rc.borrow();

                *car1_ == *car2_ && *cdr1_ == *cdr2_
            }
            _ => false,
        }
    }
}

impl<T> From<T> for LispExp
where
    T: Into<LispAtom>,
{
    fn from(t: T) -> Self {
        LispExp::Atom(t.into())
    }
}

impl From<(&LispExpRef, &LispExpRef)> for LispExp {
    fn from((car, cdr): (&LispExpRef, &LispExpRef)) -> Self {
        LispExp::Cons {
            car: car.clone(),
            cdr: cdr.clone(),
        }
    }
}

impl From<(LispExpRef, LispExpRef)> for LispExp {
    fn from((car, cdr): (LispExpRef, LispExpRef)) -> Self {
        LispExp::Cons { car, cdr }
    }
}

/// Constructors
impl LispExp {
    /// Create a LispExp::Atom::Symbol
    ///
    /// # Examples
    ///
    /// ```
    /// use pype::types::*;
    ///
    /// let e = LispExp::new_symbol("a");
    ///
    /// assert_eq!(e, LispExp::Atom(LispAtom::Symbol("a".to_string())));
    /// ```
    pub fn new_symbol<T>(e: T) -> Self
    where
        T: Into<String>,
    {
        LispExp::Atom(LispAtom::new_symbol(e))
    }
}

/// Accessors
impl LispExp {
    pub fn car(&self) -> LispExpRefStrong {
        match self {
            LispExp::Atom(_) => panic!("WrongTypeArgument: atom; car"),
            LispExp::Cons { car, .. } => car.upgrade().unwrap(),
        }
    }

    pub fn car_weak(&self) -> LispExpRef {
        match self {
            LispExp::Atom(_) => panic!("WrongTypeArgument: atom; car_weak"),
            LispExp::Cons { car, .. } => car.clone(),
        }
    }

    pub fn car_weak_ref(&self) -> &LispExpRef {
        match self {
            LispExp::Atom(_) => panic!("WrongTypeArgument: atom; car_weak_ref"),
            LispExp::Cons { car, .. } => car,
        }
    }

    pub fn cdr(&self) -> LispExpRefStrong {
        match self {
            LispExp::Atom(_) => panic!("WrongTypeArgument: atom; cdr"),
            LispExp::Cons { cdr, .. } => cdr.upgrade().unwrap(),
        }
    }

    pub fn cdr_weak(&self) -> LispExpRef {
        match self {
            LispExp::Atom(_) => panic!("WrongTypeArgument: atom; cdr_weak"),
            LispExp::Cons { cdr, .. } => cdr.clone(),
        }
    }

    pub fn cdr_weak_ref(&self) -> &LispExpRef {
        match self {
            LispExp::Atom(_) => panic!("WrongTypeArgument: atom; cdr_weak_ref"),
            LispExp::Cons { cdr, .. } => cdr,
        }
    }

    pub fn extract_args<const N: usize, const M: usize>(
        &self,
        name: &str,
        nil_exp: &LispExpRef,
    ) -> [LispExpRefStrong; M] {
        let args = self.iter_ptr().collect::<Vec<_>>();

        if !(N <= args.len() && args.len() <= M) {
            panic!(
                "WrongNumberOfArguments: {}, expected: ({}, {}), actual: {}",
                name,
                N,
                M,
                args.len()
            );
        }

        let nil_ptr = nil_exp.upgrade().unwrap();

        args.into_iter()
            .chain(std::iter::repeat(nil_ptr))
            .take(M)
            .collect::<Vec<_>>()
            .try_into()
            .expect("should be same length")
    }
}

// Iter

pub struct ConsIter {
    car: Option<LispExpRef>,
    cdr: Option<LispExpRef>,
}

impl Iterator for ConsIter {
    type Item = (LispExpRef, LispExpRef);

    fn next(&mut self) -> Option<Self::Item> {
        let car = self.car.take()?;
        let cdr = self.cdr.take()?;

        let cdr_ptr = cdr.upgrade().expect("valid reference");
        match &*cdr_ptr.borrow() {
            LispExp::Atom(_) => {
                self.car = None;
                self.cdr = None;
            }
            LispExp::Cons { car, cdr } => {
                self.car = Some(car.clone());
                self.cdr = Some(cdr.clone());
            }
        }
        Some((car, cdr))
    }
}

pub struct Iter(ConsIter);

impl Iterator for Iter {
    type Item = LispExpRef;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(car, cdr)| {
            let cdr_ptr = cdr.upgrade().unwrap();
            let x = match &*cdr_ptr.borrow() {
                LispExp::Atom(LispAtom::Symbol(sym)) if sym == "nil" => car,
                LispExp::Atom(..) => panic!("WrongTypeArgument: atom; cdr"),
                _ => car,
            };
            x
        })
    }
}

impl LispExp {
    pub fn cons_iter(&self) -> ConsIter {
        match self {
            LispExp::Atom(_) => panic!("WrongTypeArgument: atom; cons_iter"),
            LispExp::Cons { car, cdr } => ConsIter {
                car: Some(car.clone()),
                cdr: Some(cdr.clone()),
            },
        }
    }

    pub fn iter(&self) -> Iter {
        Iter(self.cons_iter())
    }

    pub fn cons_iter_ptr(&self) -> impl Iterator<Item = (LispExpRefStrong, LispExpRefStrong)> {
        self.cons_iter().map(|(car, cdr)| {
            (
                car.upgrade().expect("valid reference"),
                cdr.upgrade().expect("valid reference"),
            )
        })
    }

    pub fn iter_ptr(&self) -> impl Iterator<Item = LispExpRefStrong> {
        self.iter().map(|x| x.upgrade().expect("valid reference"))
    }
}

/// Setters
impl LispExp {
    /// Set car of the value
    ///
    /// # Examples
    /// ```
    /// use pype::types::*;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let mut arena = LispArena::default();
    /// let nil = arena.alloc_symbol("nil");
    /// let c1 = arena.alloc(1.into());
    /// let c2 = arena.alloc(2.into());
    /// let c3 = arena.alloc(3.into());
    ///
    /// let e1 = arena.alloc((&c1, &nil).into());
    /// let e2 = arena.alloc((&c2, &e1).into());
    /// let e3 = arena.alloc((&c3, &e2).into());
    /// assert_eq!(e3.upgrade().unwrap().borrow().to_string(), "(3 2 1)");
    ///
    /// let v1 = arena.alloc(42.into());
    /// let e2_ptr = e2.upgrade().unwrap();
    /// e2_ptr.borrow_mut().setcar(&v1);
    /// assert_eq!(e3.upgrade().unwrap().borrow().to_string(), "(3 42 1)");
    /// ```
    pub fn setcar<'a>(&mut self, car: &'a LispExpRef) -> &'a LispExpRef {
        match self {
            LispExp::Cons {
                car: ref mut cons_car,
                ..
            } => *cons_car = car.clone(),
            LispExp::Atom(_) => panic!("WrongTypeArgument: atom; setcar"),
        }

        car
    }

    /// Set cdr of the value
    ///
    /// # Examples
    /// ```
    /// use pype::types::*;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let mut arena = LispArena::default();
    /// let nil = arena.alloc_symbol("nil");
    /// let c1 = arena.alloc(1.into());
    /// let c2 = arena.alloc(2.into());
    /// let c3 = arena.alloc(3.into());
    ///
    /// let e1 = arena.alloc((&c1, &nil).into());
    /// let e2 = arena.alloc((&c2, &e1).into());
    /// let e3 = arena.alloc((&c3, &e2).into());
    /// assert_eq!(e3.upgrade().unwrap().borrow().to_string(), "(3 2 1)");
    ///
    /// let v1 = arena.alloc(42.into());
    /// let e2_ptr = e2.upgrade().unwrap();
    /// e2_ptr.borrow_mut().setcdr(&v1);
    /// assert_eq!(e3.upgrade().unwrap().borrow().to_string(), "(3 2 . 42)");
    /// ```
    pub fn setcdr<'a>(&mut self, cdr: &'a LispExpRef) -> &'a LispExpRef {
        match self {
            LispExp::Cons {
                cdr: ref mut cons_cdr,
                ..
            } => *cons_cdr = cdr.clone(),
            LispExp::Atom(_) => panic!("WrongTypeArgument: atom; setcdr"),
        }

        cdr
    }
}
