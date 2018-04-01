# Data 
`X-clean.csv` is a cleaned and converted version of the section related to X in [this NAIC report](http://www.naic.org/prod_serv/MSR-LB-17.pdf).
# Code
All code for processing `X-clean.csv` is in `main.rs`. No, it's not particularly clean, but the code is intended to be run only once.
# Output
`output-X.csv` lists US life insurance groups by amount in X. Note that some groups, such as "METROPOLITAN GRP," can refer to multiple companies. The breakdown is available toward the end of the previously linked NAIC report.

The `Code` column in `output-X.csv` provides a unique identifier for each US life insurance group, used as a reference to navigate the `company-X` directory. For example, in the record `NORTHWESTERN MUT GRP,10067144919,860` of `output-life-insurance.csv`, the unique identifier of 860 is given to Northwestern Mutual Group. The file `860.csv` in `company-life-insurance` then breaks down the life insurance premiums of Northwestern Mutual Group by state.
