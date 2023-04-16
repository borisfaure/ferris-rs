use core::convert::Infallible;
use keyberon::action::{d, k, l, m, Action, HoldTapAction, HoldTapConfig};
use keyberon::key_code::KeyCode::*;
use keyberon::layout::Layout;

/// Keyboard Layout type to mask the number of layers
pub type KBLayout = Layout<10, 4, 7, Infallible>;

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
const W_WIN: Action = ht!(k(LGui), k(W));
/// Win when held, or O
const O_WIN: Action = ht!(k(RGui), k(O));
/// Left Control when held, or A
const A_CTL: Action = ht!(k(LCtrl), k(A));
/// Right Control when held, or SemiColon
const SC_C: Action = ht!(k(RCtrl), k(SColon));
/// Left Shift when held, or Escape
const ESC_S: Action = ht!(k(LShift), k(Escape));
/// Right Shift when held, or Slash
const SL_S: Action = ht!(k(RShift), k(Slash));
/// Left Alt when held, or X
const X_ALT: Action = ht!(k(LAlt), k(X));
/// Left Alt when held, or .
const DOT_A: Action = ht!(k(LAlt), k(X));

/// Layer 1 (lower) when held, or Tab
const TAB_L: Action = ht!(l(1), k(Tab));

/// Layer 2 (raise) when held, or Enter
const ENT_R: Action = ht!(l(2), k(Enter));

/// Layer 3 (numbers/Fx) when held, or B
const B_NUM: Action = ht!(l(3), k(B));
/// Layer 3 (numbers/Fx) when held, or N
const N_NUM: Action = ht!(l(3), k(N));

/// Layer 4 (misc) when held, or T
const T_MI: Action = ht!(l(4), k(T));
/// Layer 4 (misc) when held, or Y
const Y_MI: Action = ht!(l(4), k(Y));

/// Layer 5 (tmux) when held, or F
const F_TX: Action = ht!(l(5), k(F));

/// Shift-Insert
const S_INS: Action = m(&[LShift, Insert].as_slice());

/// Change default layer to GAME
const GAME: Action = d(6);
/// Change default layer to BASE
const BASE: Action = d(0);

#[rustfmt::skip]
/// Layout
pub static LAYERS: keyberon::layout::Layers<10, 4, 7, Infallible> = keyberon::layout::layout! {
    { /* 0: BASE */
[ Q        {W_WIN}  E  R        {T_MI}      {Y_MI}   U        I  {O_WIN}  P      ],
[ {A_CTL}  S        D  {F_TX}   G           H        J        K  L        {SC_C} ],
[ {ESC_S}  {X_ALT}  C  V        {B_NUM}     {N_NUM}  M        ,  {DOT_A}  {SL_S} ],
[ n        n        n  {TAB_L}  Space       BSpace   {ENT_R}  n  n        n      ],
    } { /* 1: LOWER */
        [ !  #  $    '(' ')'    ^       &       {S_INS}  *      ~    ],
        [ =  -  '`'  '{' '}'    Left    PgDown  PgUp     Right  '\\' ],
        [ @  &  %    '[' ']'    n       n       Home     '\''   '"'  ],
        [ n  n  n     n  RAlt   Escape  Delete  n        n      n    ],
    } { /* 2: RAISE TODO: sequences */
        [ {BASE}  n    E  E         E           Z       U      I   O      PScreen ],
        [ A       '_'  +  &         |           Left    Down   Up  Right  PgUp    ],
        [ E       O    C  CapsLock  NumLock     N       M      ,   .      PgDown  ],
        [ n       n    n  Delete    RAlt        BSpace  Enter  n   n      n       ],
    } { /* 3: NUMBERS Fx */
        [ .  4  5  6  =        /  F1   F2   F3   F4  ],
        [ 0  1  2  3  -        *  F5   F6   F7   F8  ],
        [ ,  7  8  9  NumLock  +  F9   F10  F11  F12 ],
        [ n  n  n  t  t        t  t    n    n    n   ],
    } { /* 4: MISC TODO: mouse */
        [ Pause  {GAME}             n               R              n      n  n  n  n  n ],
        [ n      VolUp              Mute            VolDown        n      n  n  n  n  n ],
        [ n      MediaPreviousSong  MediaPlayPause  MediaNextSong  n      n  n  n  n  n ],
        [ n      n                  n               n              n      n  n  n  n  n ],
    } { /* 5: TMUX TODO: sequences */
        [ Q  W  E  R    T      Y       U      I  O  P ],
        [ A  S  D  F    G      H       J      K  L  ; ],
        [ Z  X  C  V    B      N       M      ,  .  / ],
        [ n  n  n  Tab  Space  BSpace  Enter  n  n  n ],
    } { /* 6: Gaming */
        [ Q  W  E   R        T      Y       U       I  {O_WIN}   P     ],
        [ A  S  D   F        G      H       J       K   L       {SC_C} ],
        [ Z  X  C   V        B      N       M       ,  {DOT_A}  {SL_S} ],
        [ n  n  n  {TAB_L}  Space  BSpace  {ENT_R}  n   n        n     ],
    }
};
