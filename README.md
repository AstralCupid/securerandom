# securerand

securerand is a crate that generates secure random numbers. The securerand
crate uses getrandom to fetch random numbers from the operating system. We
don't trust this randomness fully, as botched operating system RNG has been
used to compromise users in the past.

To harden the operating system entropy, we spend 15 milliseconds of every
minute (including 15 milliseconds at startup) hashing the operating system RNG
against a known salt. This takes advantage of the fact that CPUs are unstable
and will compute a different number of hahses each time, which adds entropy
because an attacker will not know exactly how many times the entropy has been
re-seeded.

It also increases the computational burden of an attacker that knows the
initial entropy and wants to compute the current state of the entropy pool.
