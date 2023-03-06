#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<10, 4, 1, ()> = keyberon::layout::layout! {
    {
        [ Q  W  E  R    T      Y       U      I  O  P ],
        [ A  S  D  F    G      H       J      K  L  ; ],
        [ Z  X  C  V    B      N       M      ,  .  / ],
        [ t  t  t  Tab  Space  BSpace  Enter  t  t  t ],
    }
};
