# one-shot-allocator

In Rust, we can override the default allocator to use more efficient
strategies.

One such strategy is the one-shot allocator that allocates, but does not free.

This is similar to a garbage-collected allocator with garbage collection
disabled used by many Java-based servers.

When we run out of memory, we restart the service.

This streategy is only used on very high performance services, embedded systems
and console games. As malloc is a large source of slowdown, for example allocating
header fields in HTTP servers, this will remove that overhead.

We use an atomic variable to make this thread safe. As memory is allocated sequentially,
cache behaviour is likely to be ideal.

## The default rust allocator.

Note that the glibc malloc is no slouch. It uses thread local memory to allocate
from a local pool. We could also use the same trick with much less overhead,
but this is just an illustration, not a perfect implementation!

On x86-64 linux, thread local access is pretty good, using the fs: segment for
local storage. It would be interesting to compare the `thread_local!` macro with
this.

## Use

```
pub fn test() {
    let _b = Box::new_in(1, MyAlloc);
}
```

Here we are using `Box::new_in` instead of `Box::new` to allocate an object.

As we have not implemented `deallocate` this will not free at the end of the scope.

We could also replace the error handling with `asm!("int 3")` to further shorten
the code.

## Generated code

https://godbolt.org/z/fGh4j433K

```
example::test:
        push    rax
        mov     rcx, qword ptr [rip + example::PTR@GOTPCREL]
        mov     eax, 16
        lock            xadd    qword ptr [rcx], rax
        lea     rcx, [rax + 4]
        cmp     rcx, 1048577
        jae     .LBB0_1
        mov     rcx, qword ptr [rip + example::MEMORY@GOTPCREL]
        mov     dword ptr [rcx + rax], 1
        pop     rax
        ret
...
```

Note there are no calls to external libraries and hence very little
cold code overhead.
