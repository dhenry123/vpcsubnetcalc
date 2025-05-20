&copy; 2025 - Damien HENRY - <https://www.mytinydc.com>

# vpcsubnetcalc

## Program Description

This program processes a given network address and generates subnets based on specified capacities.

Features:

- **Network Input**: Accepts a base network address (e.g., 172.28.0.0).
- **Subnet Capacity**: Allows users to define multiple subnets with specific capacities (e.g., "10 20 20 20"), indicating four subnets with respective capacities of 10, 20, 20, and 20.
- **Subnet Calculation**: Automatically calculates and displays the resulting subnets based on the provided capacities, ensuring efficient IP address allocation.

## Run 

- help: -h or --help
- help about network capacity: -d

### Output as text
```bash
vpcsubnetcalc -n 172.28.10.0 -s '10 20 20 20'
#or 
#vpcsubnetcalc --net 172.28.10.0 --subnets '10 20 20 20'
```
Result: 
```txt
---------------------------
VPC                  : 172.28.10.0/25
Subnet(s)            : 4
Capacities:          : [10 20 20 20]
---------------------------
VPC subnet N°1
----------->
Network              : 172.28.10.0
Mask                 : 255.255.255.240
prefix               : 28
Capacity             : 14
CIDR                 : 172.28.10.0/28
Broadcast            : 172.28.10.15
First usable address : 172.28.10.1
Last usable address  : 172.28.10.14
---------------------------
VPC subnet N°2
----------->
Network              : 172.28.10.32
Mask                 : 255.255.255.224
prefix               : 27
Capacity             : 30
CIDR                 : 172.28.10.32/27
Broadcast            : 172.28.10.63
First usable address : 172.28.10.33
Last usable address  : 172.28.10.62
---------------------------
VPC subnet N°3
----------->
Network              : 172.28.10.64
Mask                 : 255.255.255.224
prefix               : 27
Capacity             : 30
CIDR                 : 172.28.10.64/27
Broadcast            : 172.28.10.95
First usable address : 172.28.10.65
Last usable address  : 172.28.10.94
---------------------------
VPC subnet N°4
----------->
Network              : 172.28.10.96
Mask                 : 255.255.255.224
prefix               : 27
Capacity             : 30
CIDR                 : 172.28.10.96/27
Broadcast            : 172.28.10.127
First usable address : 172.28.10.97
Last usable address  : 172.28.10.126
---------------------------
```

### Output as json
```bash
vpcsubnetcalc -n 172.28.10.0 -s '10 20 20 20' --json
# or
#vpcsubnetcalc --net 172.28.10.0 --subnets '10 20 20 20' --json
```
Result: 
```json
{"vpc":"172.28.10.0/25","number_of_subnets":4,"capacities":"10 20 20 20","subnets":[{"idx":0,"network":"172.28.10.0","mask":"255.255.255.240","prefix":28,"capacity":14,"cidr":"172.28.10.0/28","broadcast":"172.28.10.15","first_usable":"172.28.10.1","last_usable":"172.28.10.14"},{"idx":1,"network":"172.28.10.32","mask":"255.255.255.224","prefix":27,"capacity":30,"cidr":"172.28.10.32/27","broadcast":"172.28.10.63","first_usable":"172.28.10.33","last_usable":"172.28.10.62"},{"idx":2,"network":"172.28.10.64","mask":"255.255.255.224","prefix":27,"capacity":30,"cidr":"172.28.10.64/27","broadcast":"172.28.10.95","first_usable":"172.28.10.65","last_usable":"172.28.10.94"},{"idx":3,"network":"172.28.10.96","mask":"255.255.255.224","prefix":27,"capacity":30,"cidr":"172.28.10.96/27","broadcast":"172.28.10.127","first_usable":"172.28.10.97","last_usable":"172.28.10.126"}]}
```

### Output as json formated with 'jq'
```bash
vpcsubnetcalc -n 172.28.10.0 -s '10 20 20 20' --json | jq
# or
#vpcsubnetcalc --net 172.28.10.0 --subnets '10 20 20 20' --json | jq
```
Result: 
```json
{
  "vpc": "172.28.10.0/25",
  "number_of_subnets": 4,
  "capacities": "10 20 20 20",
  "subnets": [
    {
      "idx": 0,
      "network": "172.28.10.0",
      "mask": "255.255.255.240",
      "prefix": 28,
      "capacity": 14,
      "cidr": "172.28.10.0/28",
      "broadcast": "172.28.10.15",
      "first_usable": "172.28.10.1",
      "last_usable": "172.28.10.14"
    },
    {
      "idx": 1,
      "network": "172.28.10.32",
      "mask": "255.255.255.224",
      "prefix": 27,
      "capacity": 30,
      "cidr": "172.28.10.32/27",
      "broadcast": "172.28.10.63",
      "first_usable": "172.28.10.33",
      "last_usable": "172.28.10.62"
    },
    {
      "idx": 2,
      "network": "172.28.10.64",
      "mask": "255.255.255.224",
      "prefix": 27,
      "capacity": 30,
      "cidr": "172.28.10.64/27",
      "broadcast": "172.28.10.95",
      "first_usable": "172.28.10.65",
      "last_usable": "172.28.10.94"
    },
    {
      "idx": 3,
      "network": "172.28.10.96",
      "mask": "255.255.255.224",
      "prefix": 27,
      "capacity": 30,
      "cidr": "172.28.10.96/27",
      "broadcast": "172.28.10.127",
      "first_usable": "172.28.10.97",
      "last_usable": "172.28.10.126"
    }
  ]
}
```

## Licence

MIT Licence see file LICENSE.txt