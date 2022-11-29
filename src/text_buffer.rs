pub struct text_buffer {
    // oh have font etc.
    // maybe i should have one 'buffer thing' and shader per thing i render
    // or maybe need separate front and back for shader because some things might use the same one
    chars: Vec<(Rect, char)>,
}

impl text_buffer {
    pub fn new
}

// should just be
pub struct SimIn {
    // frame inputs

}
pub struct SimOut {
    // render data
}
pub struct SimState {
    // retained state - entities etc
}

// then the scope of the function is the 'default workspace'. its just you fuck it up by having other ones. but you can transform it to have a single workspace
// then you could have static data like the definitions of the entities and shit etc. doable with functions (can go in workspace since its immutable, ie doesnt have to be stored anywhere, namespace only)
// but thats boilerplate. i guess thats a reason to be dynamic, do it once. definition and usage in the same place

// static data has traits to be renderable etc.

// could these all be rows / columns, so I can say: player radius: 0.0, gets timesed by 0.1, 