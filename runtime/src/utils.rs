use core::marker::PhantomData;
use frame_support::traits::{ChangeMembers, InitializeMembers};

pub struct MembersNotifyBoth<T1, T2> {
    _marker: PhantomData<(T1, T2)>,
}
impl<A, T1, T2> ChangeMembers<A> for MembersNotifyBoth<T1, T2>
where
    A: Clone + Ord,
    T1: ChangeMembers<A>,
    T2: ChangeMembers<A>,
{
    fn change_members_sorted(incoming: &[A], outgoing: &[A], sorted_new: &[A]) {
        T1::change_members_sorted(incoming, outgoing, sorted_new);
        T2::change_members_sorted(incoming, outgoing, sorted_new);
    }
}
impl<A, T1, T2> InitializeMembers<A> for MembersNotifyBoth<T1, T2>
where
    A: Clone + Ord,
    T1: InitializeMembers<A>,
    T2: InitializeMembers<A>,
{
    fn initialize_members(members: &[A]) {
        T1::initialize_members(members);
        T2::initialize_members(members);
    }
}
