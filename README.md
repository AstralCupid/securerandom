# securerand

NOTE: I am still learning rust and haven't figured out how to implement the
entropy hardening features yet. They will be available soon.

securerand is a crate that generates secure random numbers. The securerand
crate uses getrandom to fetch random numbers from the operating system. We then
take steps to harden this entropy, as weak entropy from the operating system
has been used to successfully exploit users in the past. Our hardening strategy
is not itself sufficient to provide secure entropy, it is merely a technique
for making operating system entropy stronger. Our technique does not risk
reducing the entropy of our randomness pool.

To harden the operating system entropy, we spend 15 milliseconds of every
minute (including 15 milliseconds at startup) hashing the operating system RNG
against a salt. This takes advantage of the fact that CPUs are unstable and
will compute a different number of hahses each time, which adds entropy. The
number of iterations is then used to seed the salt for the next run.

This hardening on its own provides roughly [NOTE: HAVENT TESTED] bits of
entropy per minute. If you wish to generate additional entropy at runtime, call
secure\_harden. secure\_harden will run for three seconds which will provide
roughly [NOTE: HAVENT TESTED] bits of additional entropy. It should be noted
that 'roughly' is poorly defined - the amount of entropy provided will vary
greatly depending on the system that is being used.
