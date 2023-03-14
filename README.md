# one-shot-allocator

In Rust, we can override the default allocator to use more efficient
strategies.

One such strategy is the one-shot allocator that allocates, but does not free.

This is similar to a garbage-collected allocator with garbage collection
disabled use by many Java-based servers.

When we run out of memory, we restart the service.

This streategy is only used on very high performance services, embedded systems
and console games. As malloc is a large source of slowdown, for example allocating
header fields in HTTP servers, this will remove that overhead.

We use an atomic variable to make this thread safe. This is probably cheaper
than using thread local memory on many platforms which call the OS to get
the TLS pointer. As memory is allocated sequentially, cache behaviour is
likely to be ideal.

A smarter allocator would use pool allocation to pull the next block off
a linked list and page allocate when the list is exhausted. Freeing can be done
by adding the object back on the list. Some quantisation of block sizes is
required to make this more efficient as otherwise the number of pools will grow.

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

