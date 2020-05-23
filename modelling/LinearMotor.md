Linear Motor Modelling
======================

## E1: Hallbach Array
Question: Does it make sense to use a hallbach array for the stator?

Answer: Yes, it's about twice as strong as the NS alignment with the same period

see [Experiment 1](E1/E1_Hallbach.md)


## E2: Iron Sheet under Hallbach Array
Question: Does adding an iron sheet under the halbach array provide any benefit?

Answer: No, it does not make any difference at all

see [Experiment 2](E2/E2_IronSheet.md)


## E3: Effect of Magnet Spacing in Hallbach Array
Question: How much does adding empty space between the strator magnets lessen the magnetic field?

Motivation: obviously using less magnets would be beneficial because it's less expensive

Answer: Field strength changes approx. linearly with gap with ~0.083 T/mm and at 6mm its no longer continuous but has a "bump".

see [Experiment 3](E3/E3_Spacing.md)

## E4: Solenoid Orientation
Question: Which orientation of the soleniod does make most sense in regards to providing x-axis force?

Answer: horizontally aligned solenoid (N facing left/right) is better by approx. 20%

see [Experiment 4](E4/E4_Orientation.md)

## E5: Impact of Solenoid length on fields
Question: How much does the length of the solenoid affect the x-axis force given the same number of turns per mm? (in general)

(depends on the best orientation for E4)

Answer: TBD

## E6: Solenoid Length
Question: Whats the optimal length of a solenoid given a stator layout? (i.e. relative to the period of the stator)

(depends on the results of E3, E4 and E5)

Answer: TBD

## E7: Linear Motor as Levitation Control
Question: Is it feasible to also fine-control the levitation height (air-gap) by applying adjustments to the solenoid? Where are the respective x-axis positions?

Motivation: would save or a least simplify a seperate system to control levitation height

Idea: Depending of the solenoid orientation there might be a phase where a single solenoid primary affects the y-axis. By fine-controlling the direction and magnitude of the current it can add/reduce the levitation for a short moment (until it travels further on the x-axis).

Answer: TBD

(might be derivable from results of E4 and E6)


## E8: Multiple Solenoids
Question: How to place the solenoids relative to each other to provider continuous x-axis power without "dead" points?

(depends on E6)

Answer: TBD
