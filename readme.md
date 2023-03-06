# What is it ?

A parser for Chenhao Tan's dataset of r/changemyview :
https://chenhaot.com/pages/changemyview.html

# How to use it ?

Download the dataset:
```shell
wget https://chenhaot.com/data/cmv/cmv.tar.bz2
```

Extract the dataset:
```shell
tar -xf cmv.tar.bz2
lbzip2 -d all/train_period_data.jsonlist.bz2
lbzip2 -d all/heldout_period_data.jsonlist.bz2
```
