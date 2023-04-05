use core::convert::Infallible;
use keyberon::action::{d, k, l, m, Action, HoldTapAction, HoldTapConfig};
use keyberon::key_code::KeyCode::*;
use keyberon::layout::Layout;

/// Keyboard Layout type to mask the number of layers
pub type KBLayout = Layout<10, 4, 7, Infallible>;

const TIMEOUT: u16 = 200;
const TAP_HOLD_INTERVAL: u16 = 0;

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

const W_WIN: Action = ht!(k(LGui), k(W));
const O_WIN: Action = ht!(k(RGui), k(O));
const A_CTL: Action = ht!(k(LCtrl), k(A));
const SC_C: Action = ht!(k(RCtrl), k(SColon));
const ESC_S: Action = ht!(k(LShift), k(Escape));
const SL_S: Action = ht!(k(RShift), k(Slash));
const X_ALT: Action = ht!(k(LAlt), k(X));
const DOT_A: Action = ht!(k(LAlt), k(X));

const TAB_L: Action = ht!(l(1), k(Tab));
const ENT_R: Action = ht!(l(2), k(Enter));

const B_NUM: Action = ht!(l(3), k(B));
const N_NUM: Action = ht!(l(3), k(N));

const T_MI: Action = ht!(l(4), k(T));
const Y_MI: Action = ht!(l(4), k(Y));
const F_TX: Action = ht!(l(5), k(F));

const S_INS: Action = m(&[LShift, Insert].as_slice());

const GAME: Action = d(6);
const BASE: Action = d(0);

#[rustfmt::skip]
/// Default layout
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
        [ Q  W  E  R        T      Y       U        I  {O_WIN}  P ],
        [ A  S  D  F        G      H       J        K  L        {SC_C} ],
        [ Z  X  C  V        B      N       M        ,  {DOT_A}  {SL_S} ],
        [ n  n  n  {TAB_L}  Space  BSpace  {ENT_R}  n  n        n ],
    }
};
