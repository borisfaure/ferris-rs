use core::convert::Infallible;
use keyberon::action::{d, k, l, m, Action, HoldTapAction, HoldTapConfig};
use keyberon::key_code::KeyCode::*;
use keyberon::layout::Layout;

/// Keyboard Layout type to mask the number of layers
pub type KBLayout = Layout<10, 4, 8, Infallible>;

/// Timeout to consider a key as held
const TIMEOUT: u16 = 200;
/// Disable tap_hold_interval
const TAP_HOLD_INTERVAL: u16 = 0;

/// Helper to create a HoldTapAction
macro_rules! ht {
    ($h:expr, $t:expr) => {
        Action::HoldTap(&HoldTapAction {
            timeout: TIMEOUT,
            tap_hold_interval: TAP_HOLD_INTERVAL,
            config: HoldTapConfig::Default,
            hold: $h,
            tap: $t,
        })
    };
}

/// HoldTap(KeyCode(LShift), KeyCode(A))
const LSA: Action = ht!(k(LShift), k(A));
/// HoldTap(Layer(5), KeyCode(S))
const S5: Action = ht!(l(5), k(S));
/// HoldTap(Layer(1), KeyCode(D))
const D1: Action = ht!(l(1), k(D));
/// HoldTap(Layer(3), KeyCode(F))
const F3: Action = ht!(l(3), k(F));
/// HoldTap(Layer(4), KeyCode(J))
const J4: Action = ht!(l(4), k(J));
/// HoldTap(Layer(2), KeyCode(K))
const K2: Action = ht!(l(2), k(K));
/// HoldTap(Layer(6), KeyCode(L))
const L6: Action = ht!(l(6), k(L));
/// HoldTap(Layer(7), KeyCode(Space))
const L7: Action = ht!(l(7), k(Space));

/// HoldTap(KeyCode(LShift), KeyCode(SColon))
const LSSC: Action = ht!(k(LShift), k(SColon));
/// HoldTap(KeyCode(LCtrl), KeyCode(X))
const LCX: Action = ht!(k(LCtrl), k(X));
/// HoldTap(KeyCode(LAlt), KeyCode(C)),
const LAC: Action = ht!(k(LAlt), k(C));
/// HoldTap(KeyCode(LAlt), KeyCode(Comma))
const LACM: Action = ht!(k(LAlt), k(Comma));
/// HoldTap(KeyCode(LCtrl), KeyCode(Dot))
const LCD: Action = ht!(k(LCtrl), k(Dot));
/// MultipleKeyCodes([LCtrl, LAlt])
const CA: Action = m(&[LCtrl, LAlt].as_slice());
/// MultipleKeyCodes([LCtrl, LAlt, LShift])
const CAS: Action = m(&[LCtrl, LAlt, LShift].as_slice());
/// HoldTap(KeyCode(RCtrl), KeyCode(Dot))
const RCD: Action = ht!(k(RCtrl), k(Dot));
/// HoldTap(KeyCode(RAlt), KeyCode(Comma))
const RACM: Action = ht!(k(RAlt), k(Comma));

/// DefaultLayer(0)
const DF0: Action = d(0);
/// DefaultLayer(1)
const DF1: Action = d(1);

#[rustfmt::skip]
/// Layout
pub static LAYERS: keyberon::layout::Layers<10, 4, 8, Infallible> = keyberon::layout::layout! {
    { // 0: Base Layer
[Q     W     E     R    T      Y    U    I      O      P      ],
[{LSA} {S5}  {D1}  {F3} G      H    {J4} {K2}   {L6}   {LSSC} ],
[Z     {LCX} {LAC} V    B      N    M    {LACM} {LCD}  /      ],
[n     n     n     0    BSpace {L7} 1    n      n      n      ],
    } { // 1: Mouse TODO: mouse support
        [t t t t t    t n n n t],
        [t n n n t    t n n n n],
        [t t t t t    t n n n t],
        [n n n t t    t t n n n],
    } { // 2: Navigation
        [t    t    PgUp   t     t        t t    t t    t     ],
        [Left Up   Down   Right t        t LGui n {CA} {CAS} ],
        [t    Home PgDown End   t        t t    t t    t     ],
        [n    n    n      t     t        t t    n n    n     ],
    } { // 3: Right Symbols
        [t t t t t     t '_'   |   '\'' t],
        [^ * & n t     #  ~    /   '"'  $],
        [t t t t t     t  -   '\\'  .   t],
        [n n n n t     t  n    n    n   n],
    } { // 4: Left Symbols
        [ t    :    <    >            ;     t  t          t t t],
        ['{'  '}'  '('  ')'           @     t  n          = + %],
        [ t    !   '['  ']'           t     t  t          t t t],
        [ n    n    n   MediaVolDown  t     t  MediaVolUp n n n],
    } { // 5: Function keys
        [t t t    t t      t F7 F8 F9 F10],
        [t n {CA} t t      t F4 F5 F6 F11],
        [t t t    t t      t F1 F2 F3 F12],
        [n n n    t t      t t  n  n  n],
    } { // 6: Numbers
        [/ 7 8 9 +     t t t t t],
        [0 1 2 3 -     t t t n t],
        [* 4 5 6 =     t t t t t],
        [n n n t t     t t n n n],
    } { // 7: Always accessible
        [t t : Escape t    t     t    t      t     Delete],
        [t % / Enter  t    {DF1} LGui t      t     t     ],
        [t t t !      t    {DF0} t    {RACM} {RCD} n     ],
        [n n n t      Tab  n     t    n      n     n     ],
    }
};
