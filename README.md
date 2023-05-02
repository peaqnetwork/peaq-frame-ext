# peaq-frame-ext

It stands for *Peaq's Frame Extension*, whereas the term frame belongs to the substrate-framework.
We seperate definitions and implementations, that are very generic for substrate-pallets out into
this crate. In case someone in the open-source-world wants to use some of our pallets, that depend
on this crate's trait definitions, you only have to import this crate and not other parts from the
peaq-network-node.

The main goal behind this is independence. At Peaq we try to make pallets as independent and generic
as possible. Sometimes pallets have to be connected, where tight- and loose-coupling are the two 
offered approaches by substrate. This crate extends the concept of loose coupling by defining traits,
that are not contained within substrate.

Currently there is only the averaging-module that contains several traits for pallets, that are
somehow calculating an average-value of a data series. In Peaq the pallet-block-reward and the 
parachain-staking-pallet do use these traits.

Have a look in the Rust code's documentation for further details.