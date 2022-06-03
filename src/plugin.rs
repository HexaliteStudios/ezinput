//! The plugin that need to be registered in order to start receiving and handling input given
//! by the `bevy_input` API.

use std::marker::PhantomData;

use crate::prelude::*;
use bevy::input::InputSystem;
use bevy::prelude::*;
use bevy::utils::Instant;

#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub struct InputHandlingTickSystem;

pub struct EZInputPlugin<Keys>
where
    Keys: BindingTypeView,
{
    phantom_keys: PhantomData<Keys>,
}

impl<Keys> Default for EZInputPlugin<Keys>
where
    Keys: BindingTypeView,
{
    fn default() -> Self {
        Self {
            phantom_keys: PhantomData,
        }
    }
}

impl<Keys> Plugin for EZInputPlugin<Keys>
where
    Keys: BindingTypeView,
{
    fn build(&self, app: &mut App) {
        #[inline]
        fn add_handling_system<Params>(
            app: &mut App,
            func: impl ParallelSystemDescriptorCoercion<Params>,
        ) {
            app.add_system_to_stage(
                CoreStage::PreUpdate,
                func.before(InputHandlingTickSystem).after(InputSystem),
            );
        }
        app.add_system_to_stage(
            CoreStage::PreUpdate,
            tick_system::<Keys>.label(InputHandlingTickSystem),
        );
        add_handling_system(
            app,
            keyboard_input_system::<Keys>.label(KeyboardInputHandlingSystem),
        );
        add_handling_system(
            app,
            mouse_input_system::<Keys>.label(MouseInputHandlingSystem),
        );
        add_handling_system(
            app,
            gamepad_input_system::<Keys>.label(GamepadInputHandlingSystem),
        );
    }
}

/// Tick every input system to update the press state for the current time, letting the input view know the press
/// state for the action.
fn tick_system<Keys>(mut query: Query<&mut InputView<Keys>>)
where
    Keys: BindingTypeView,
{
    #[inline]
    fn update_time</*T,*/ F>(state: &PressState, exec: F)
    where
        //T: BindingTypeView,
        F: FnOnce(),
    {
        if let PressState::Pressed {
            started_pressing_instant,
        } = state
        {
            if started_pressing_instant.is_none() {
                exec();
            }
        }
    }
    for mut view in query.iter_mut() {
        for (rcv, state) in view.receiver_states.clone().iter() {
            update_time(&state.press, || {
                view.set_axis_value(
                    *rcv,
                    state.value,
                    PressState::Pressed {
                        started_pressing_instant: Some(Instant::now()),
                    },
                )
            });
        }
    }
}
