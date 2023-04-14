use core::convert::Infallible;
use keyberon::action::{d, k, l, m, Action, HoldTapAction, HoldTapConfig};
use keyberon::key_code::KeyCode::*;
use keyberon::layout::Layout;

/// Keyboard Layout type to mask the number of layers
pub type KBLayout = Layout<10, 4, 9, Infallible>;

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

/// HoldTap(KeyCode(LAlt), KeyCode(M))
const LAM: Action = ht!(k(LAlt), k(M));
/// HoldTap(KeyCode(LAlt), KeyCode(Comma))
const LACm: Action = ht!(k(LAlt), k(Comma));
/// HoldTap(KeyCode(LAlt), KeyCode(Dot))
const LADot: Action = ht!(k(LAlt), k(Dot));
/// HoldTap(KeyCode(LAlt), KeyCode(G))
const LAG: Action = ht!(k(LAlt), k(G));

/// HoldTap(KeyCode(LCtrl), KeyCode(BSpace))
const LCBSp: Action = ht!(k(LCtrl), k(BSpace));
/// HoldTap(KeyCode(LCtrl), KeyCode(Dot))
const LCDot: Action = ht!(k(LCtrl), k(Dot));
/// HoldTap(KeyCode(LCtrl), KeyCode(F))
const LCF: Action = ht!(k(LCtrl), k(F));
/// HoldTap(KeyCode(LCtrl), KeyCode(X))
const LCX: Action = ht!(k(LCtrl), k(X));

/// HoldTap(KeyCode(LShift), KeyCode(A))
const LSA: Action = ht!(k(LShift), k(A));
/// HoldTap(KeyCode(LShift), KeyCode(I))
const LSI: Action = ht!(k(LShift), k(I));
/// HoldTap(KeyCode(LShift), KeyCode(O))
const LSO: Action = ht!(k(LShift), k(O));
/// HoldTap(KeyCode(LShift), KeyCode(R))
const LSR: Action = ht!(k(LShift), k(R));

/// HoldTap(Layer(2), KeyCode(H))
const H2: Action = ht!(l(2), k(H));
/// HoldTap(Layer(2), KeyCode(T))
const T2: Action = ht!(l(2), k(T));
/// HoldTap(Layer(3), KeyCode(A)),
const A3: Action = ht!(l(3), k(A));
/// HoldTap(Layer(3), KeyCode(E))
const E3: Action = ht!(l(3), k(E));
/// HoldTap(Layer(4), KeyCode(H))
const H4: Action = ht!(l(4), k(H));
/// HoldTap(Layer(4), KeyCode(T))
const T4: Action = ht!(l(4), k(T));
/// HoldTap(Layer(5), KeyCode(N))
const N5: Action = ht!(l(5), k(N));
/// HoldTap(Layer(6), KeyCode(S))
const S6: Action = ht!(l(6), k(S));
/// HoldTap(Layer(7), KeyCode(I)),
const I7: Action = ht!(l(7), k(I));
/// HoldTap(Layer(7), KeyCode(O))
const O7: Action = ht!(l(7), k(O));
/// HoldTap(Layer(8), KeyCode(Space))
const Sp8: Action = ht!(l(8), k(Space));

/// MultipleKeyCodes([LCtrl, LAlt])
const LCA: Action = m(&[LCtrl, LAlt].as_slice());
/// MultipleKeyCodes([LCtrl, LAlt, LShift])
const LCAS: Action = m(&[LCtrl, LAlt, LShift].as_slice());
/// MultipleKeyCodes([LCtrl, LAlt, Up])
const LCAUp: Action = m(&[LCtrl, LAlt, Up].as_slice());
/// MultipleKeyCodes([LCtrl, LAlt, Down])
const LCADn: Action = m(&[LCtrl, LAlt, Down].as_slice());
/// MultipleKeyCodes([LCtrl, LShift, C])
const LCSC: Action = m(&[LCtrl, LShift, C].as_slice());
/// MultipleKeyCodes([LCtrl, LShift, V])
const LCSV: Action = m(&[LCtrl, LShift, V].as_slice());
/// MultipleKeyCodes([LCtrl, Tab])
const LCTab: Action = m(&[LCtrl, Tab].as_slice());
/// MultipleKeyCodes([LCtrl, LShift, Tab])
const LCSTab: Action = m(&[LCtrl, LShift, Tab].as_slice());
/// HoldTap(KeyCode(RCtrl), KeyCode(Dot))
const RCD: Action = ht!(k(RCtrl), k(Dot));
#[allow(non_upper_case_globals)]
/// HoldTap(KeyCode(RAlt), KeyCode(Comma))
const RACm: Action = ht!(k(RAlt), k(Comma));

/// DefaultLayer(0)
const DL0: Action = d(0);
/// DefaultLayer(1)
const DL1: Action = d(1);
/// DefaultLayer(2)
const DL2: Action = d(2);

#[rustfmt::skip]
/// Layout
pub static LAYERS: keyberon::layout::Layers<10, 4, 9, Infallible> = keyberon::layout::layout! {
    { // 0: Base Layer
        [ Q     D     R     W       B          J     F       U      P      SColon],
        [{LSA} {S6}  {H2}  {T4}     G          Y    {N5}    {E3}   {O7}    {LSI} ],
        [ Z    {LCX} {LAM}  C       V          K     L      {LACm} {LCDot}  /    ],
        [ n     n     n    {LCAUp} BSpace     {Sp8} {LCADn}  n      n       n    ],
    } { // 1
        [ Q     C     M     Y      :       Z       W       ,       U      J   ],
        [{LSR} {S6}  {T2}  {H4}    X       t      {N5}    {A3}    {I7}   {LSO}],
        [ B    {LCF} {LAG}  D      V       Escape  L      {LADot} {LCBSp} K   ],
        [ n     n     n    {LCAUp} E       {Sp8}  {LCADn}  n       n      n   ],
    } { // 2
        [t  {LCSC} t  {LCSV}    t          t   n      n  n  t],
        [t   n     n   n        t          t   n      n  n  n],
        [t   t     t   t        t          t   n      n  n  t],
        [n   n     n  {LCSTab}  t          t  {LCTab} n  n  n],
    } { // 3: Navigation
        [t      t    PgUp     t     t      t   t    t   t      t    ],
        [Left  Up    Down    Right  t      t  LGui  n  {LCA}  {LCAS}],
        [t     Home  PgDown  End    t      t   t    t   t      t    ],
        [n      n     n       n     t      t   n    n   n      n    ],
    } { // 4: Right Symbols
        [t  t  t  t  t    t  '_'  |   '\'' t],
        [^  *  &  n  t    #   ~  /   '"'  $],
        [t  t  t  t  t    t   - '\\' '`'  t],
        [n  n  n  n  t    t   n  n    n   n],
    } { // 5: Left Symbols
        [ t   :   <   >             ;      t  t          t t t],
        ['{' '}' '(' ')'            @      t  n          = + %],
        [ t   !  '[' ']'            t      t  t          t t t],
        [ n   n   n   MediaVolDown  t      t  MediaVolUp n n n],
    } { // 6: Functions
        [t  t   t    t  t    t  F7  F8  F9  F10],
        [t  t  {LCA} t  t    t  F4  F5  F6  F11],
        [t  t   t    t  t    t  F1  F2  F3  F12],
        [n  n   n    t  t    t  t   n   n   n  ],
    } { // 7: Numbers
        [/ 7 8 9 +     t t t t t],
        [0 1 2 3 -     t t t n t],
        [* 4 5 6 =     t t t t t],
        [n n n t t     t t n n n],
    } { // 8: Always accessible
        [t Escape :  t    t  {DL2}  t    t      t    Delete],
        [t  %     / Enter !  {DL1} LGui  t      t    t     ],
        [t  t     t  t    t  {DL0}  t   {RACm} {RCD} n     ],
        [n  n     n  t   Tab   n    t    n      n    n     ],
    }
};
