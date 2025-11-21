# Architecture

## Top
> Each file reprensents the user entrypoint

Code base : 
- `entrypoint.py`
- `serverless-fnc.py`
- `background-svc.py`
- `__init__`

## Fragment
> Fragement represents a File from your OS

## Function
> Function represents a C/C++/Python/Java function

Contains : 
- (name): Function Name
- (lang): Programming Language developped in
- (arguments): List of arguments that the function takes as input
- (output): The primitive returned by the function


# Schema

[File hosted on OS] -(discovery::Discover::lookup)> [Fragment] -(fragment::Fragment::extract)> [Vec of Functions]