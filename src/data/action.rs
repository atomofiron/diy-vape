pub enum Action {
    Power(Increment),
    Limit(Increment),
    Resistance(Increment),
    Brightness(Increment),
}

type Increment = bool;
