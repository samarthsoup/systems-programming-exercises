# line editor
impl. in rust using only the standard library

### run the file using:
<code>cargo run -- {filename}</code>

## commands:
> p
>
> prints lines from file
>
> accepts either 0, 1, or 2 arguments that have to be positive integers
>> p: prints the first line
>
>> p $n: prints the nth line
>
>> p $n1 $n2: prints lines from n1 to n2
>

> i
>
> inserts some input into the file
>
> input is taken by going into input mode
>
>accepts 1 argument that has to be a positive integer
> 
>> i $n: inserts all input at nth line
>

> a
> 
> appends some input to end of file
>
> input is taken by going into input mode
> 
> accepts no arguments
> 
>> a: appends all input to the end of the file
>

> d
>
> deletes lines from file
>
> accepts either 0, 1, or 2 arguments that have to be positive integers
>> d: deletes the first line
>
>> d $n: deletes the nth line
>
>> d $n1 $n2: deletes lines from n1 to n2
>

> m
>
> moves lines 
>
> accepts either 2 or 3 arguments that have to be positive integers
>> m $n1 $n2: moves line from n1 to n2
>
>> m $n1 $n2 $n3: moves lines n1..n2 to n3
>

> f
>
> finds query string(case-sensitive) in file, and returns lines that contain it
>
> accepts 1 argument that is the query string
>> f $x: finds x in lines and returns those lines that have it
>

> s
> 
> saves file
>
> accepts 0 arguments
>> s: saves changes to file
>

> q
>
> quits the program
>
> accepts 0 arguments
>> q: quits the program
>