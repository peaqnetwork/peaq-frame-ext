//! Currency based pallet-mockup for ProvidesAverage-traits.

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

/// Generic Average-Selector for an arbitrary mockup.
#[derive(PartialEq, Eq, Copy, Clone, Encode, Debug, Decode, Default, TypeInfo, MaxEncodedLen)]
pub enum AverageSelector {
    #[default]
    Whatever,
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

    use frame_support::{pallet_prelude::*, traits::Currency};
    use sp_runtime::traits::Zero;

    use crate::averaging::*;
    use super::AverageSelector;

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Constant vector with OnAverageUpdate implementations to be updated.
        type AvgChangedNotifier: AvgChangedNotifier;
        /// Type of used currency.
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::storage]
    #[pallet::getter(fn get_average)]
    pub(crate) type Average<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub avg_init: BalanceOf<T>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> GenesisConfig<T> {
            GenesisConfig {
                avg_init: BalanceOf::<T>::zero(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            Average::<T>::put(self.avg_init);
        }
    }

    impl<T: Config> Pallet<T> {
        /// Update the internal average-value for testing purposes.
        pub fn update(next_avg: BalanceOf<T>, f: impl Fn(BalanceOf<T>) -> BalanceOf<T>) {
            Average::<T>::mutate(|avg| {
                *avg = f(next_avg);
            });
        }

        /// Pass through method to simplify testing with your own implementations.
        pub fn notify_clients() {
            T::AvgChangedNotifier::notify_clients();
        }
    }

    impl<T: Config> ProvidesAverage<BalanceOf<T>> for Pallet<T> {
        fn get_average() -> BalanceOf<T> {
            Average::<T>::get()
        }
    }

    impl<T: Config> ProvidesAverages<BalanceOf<T>, AverageSelector> for Pallet<T> {
        fn get_average_by(_s: AverageSelector) -> BalanceOf<T> {
            Average::<T>::get()
        }
    }

    impl<T: Config> ProvidesAverageFor<BalanceOf<T>, AverageSelector> for Pallet<T> {
        fn get_average_for(_r: AverageSelector) -> BalanceOf<T> {
            Average::<T>::get()
        }
    }

    impl<T: Config> ProvidesAveragesFor<BalanceOf<T>, AverageSelector, AverageSelector> for Pallet<T> {
        fn get_average_for_by(_s: AverageSelector, _r: AverageSelector) -> BalanceOf<T> {
            Average::<T>::get()
        }
    }
}
