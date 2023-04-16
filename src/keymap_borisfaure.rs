use core::convert::Infallible;
use keyberon::action::{d, k, l, m, Action, HoldTapAction, HoldTapConfig};
use keyberon::key_code::KeyCode::*;
use keyberon::layout::Layout;

/// Keyboard Layout type to mask the number of layers
pub type KBLayout = Layout<10, 4, 9, Infallible>;

/// Helper to create keys shifted
macro_rules! s {
    ($k:ident) => {
        m(&[LShift, $k].as_slice())
    };
}

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

/// Win when held, or W
const HT_W_W: Action = ht!(k(LGui), k(W));
/// Win when held, or O
const HT_W_O: Action = ht!(k(RGui), k(O));
/// Left Control when held, or A
const HT_C_A: Action = ht!(k(LCtrl), k(A));
/// Left Control when held, or Shift-A
const HT_C_SA: Action = ht!(k(LCtrl), s!(A));
/// Right Control when held, or SemiColon
const HT_C_SC: Action = ht!(k(RCtrl), k(SColon));
/// Left Shift when held, or Escape
const HT_S_ESC: Action = ht!(k(LShift), k(Escape));
/// Right Shift when held, or Slash
const HT_S_SL: Action = ht!(k(RShift), k(Slash));
/// Left Alt when held, or X
const HT_A_X: Action = ht!(k(LAlt), k(X));
/// Left Alt when held, or .
const HT_A_DOT: Action = ht!(k(LAlt), k(Dot));

/// Layer 1 (lower) when held, or Tab
const HT_1_TAB: Action = ht!(l(1), k(Tab));

/// Layer 2 (raise) when held, or Enter
const HT_2_ENT: Action = ht!(l(2), k(Enter));

/// Layer 3 (numbers/Fx) when held, or B
const HT_3_B: Action = ht!(l(3), k(B));
/// Layer 3 (numbers/Fx) when held, or N
const HT_3_N: Action = ht!(l(3), k(N));

/// Num Mode
const NUM: Action = ma(&[k(NumLock), d(4)].as_slice());
/// Unset Num Mode
const UNNUM: Action = ma(&[k(NumLock), d(0)].as_slice());

/// Layer 5 (misc) when held, or T
const HT_5_T: Action = ht!(l(5), k(T));
/// Layer 5 (misc) when held, or Y
const HT_5_Y: Action = ht!(l(5), k(Y));

/// Layer 6 (tmux) when held, or F
const HT_6_F: Action = ht!(l(6), k(F));

/// Shift-Insert
const S_INS: Action = m(&[LShift, Insert].as_slice());

/// A shortcut to create a `Action::MultipleActions`, useful to
/// create compact layout.
const fn ma<T, K>(actions: &'static &'static [Action<T, K>]) -> Action<T, K> {
    Action::MultipleActions(actions)
}

/// Caps Mode
const CAPS: Action = ma(&[k(CapsLock), d(8)].as_slice());
/// Unset Caps Mode
const UNCAPS: Action = ma(&[k(CapsLock), d(0)].as_slice());

/// Change default layer to GAME
const GAME: Action = d(7);
/// Change default layer to BASE
const BASE: Action = d(0);

#[rustfmt::skip]
/// Layout
pub static LAYERS: keyberon::layout::Layers<10, 4, 9, Infallible> = keyberon::layout::layout! {
    { /* 0: BASE */
[  Q         {HT_W_W}  E   R         {HT_5_T}    {HT_5_Y}   U          I  {HT_W_O}     P        ],
[ {HT_C_A}    S        D  {HT_6_F}    G           H         J          K   L          {HT_C_SC} ],
[ {HT_S_ESC} {HT_A_X}  C   V         {HT_3_B}    {HT_3_N}   M          ,  {HT_A_DOT}  {HT_S_SL} ],
[  n          n        n  {HT_1_TAB}  Space       BSpace   {HT_2_ENT}  n   n           n        ],
    } { /* 1: LOWER */
        [ !  #  $    '(' ')'    ^       &       {S_INS}  *      ~    ],
        [ =  -  '`'  '{' '}'    Left    PgDown  PgUp     Right  '\\' ],
        [ @  &  %    '[' ']'    n       n       Home     '\''   '"'  ],
        [ n  n  n     n  RAlt   Escape  Delete  n        n      n    ],
    } { /* 2: RAISE TODO: sequences */
        [ {BASE}  n    E   E        E           Z       U      I   O      PScreen ],
        [ A       '_'  +   &        |           Left    Down   Up  Right  PgUp    ],
        [ E       O    C  {CAPS}   {NUM}        N       M      ,   .      PgDown  ],
        [ n       n    n  Delete    RAlt        BSpace  Enter  n   n      n       ],
    } { /* 3: NUMBERS Fx */
        [ .  4  5   6          =         /       F1   F2   F3   F4  ],
        [ 0  1  2   3          -         *       F5   F6   F7   F8  ],
        [ ,  7  8   9          {NUM}     +       F9   F10  F11  F12 ],
        [ n  n  n  {HT_1_TAB}  Space    BSpace  {HT_2_ENT}    n    n    n   ],
    } { /* 4: NUMBERS Fx Lock */
        [ .  4  5   6          =         /       F1   F2   F3   F4  ],
        [ 0  1  2   3          -         *       F5   F6   F7   F8  ],
        [ ,  7  8   9          {UNNUM}   +       F9   F10  F11  F12 ],
        [ n  n  n  {HT_1_TAB}  Space    BSpace  {HT_2_ENT}    n    n    n   ],
    } { /* 5: MISC TODO: mouse */
        [ Pause  {GAME}             n               R              n      n  n  n  n  n ],
        [ n      VolUp              Mute            VolDown        n      n  n  n  n  n ],
        [ n      MediaPreviousSong  MediaPlayPause  MediaNextSong  n      n  n  n  n  n ],
        [ n      n                  n               n              n      n  n  n  n  n ],
    } { /* 6: TMUX TODO: sequences */
        [ Q  W  E  R    T      Y       U      I  O  P ],
        [ A  S  D  F    G      H       J      K  L  ; ],
        [ Z  X  C  V    B      N       M      ,  .  / ],
        [ n  n  n  Tab  Space  BSpace  Enter  n  n  n ],
    } { /* 7: Gaming */
        [ Q  W  E   R           T      Y       U          I  {HT_W_O}     P       ],
        [ A  S  D   F           G      H       J          K   L         {HT_C_SC} ],
        [ Z  X  C   V           B      N       M          ,  {HT_A_DOT} {HT_S_SL} ],
        [ n  n  n  {HT_1_TAB}  Space  BSpace  {HT_2_ENT}  n   n          n        ],
    } { /* 8: Caps */
[ {s!(Q)}   {s!(W)}  {s!(E)}  {s!(R)}  {s!(T)}         {s!(Y)}   {s!(U)}     {s!(I)}  {s!(O)}   {s!(P)}   ],
[ {HT_C_SA} {s!(S)}  {s!(D)}  {s!(F)}  {s!(G)}         {s!(H)}   {s!(J)}     {s!(K)}  {s!(L)}   {HT_C_SC} ],
[ {UNCAPS}  {s!(X)}  {s!(C)}  {s!(V)}  {s!(B)}         {s!(N)}   {s!(M)}      ,        .         /        ],
[  n         n        n        '_'      Space           BSpace   {HT_2_ENT}   n        n         n        ],
    }
};
