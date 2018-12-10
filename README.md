# smart - dynamically dispatched smart pointers

This library lets you construct a smart pointer dynamically from one of our
different pointers which implement shared ownership. It has both a threadsafe
(`SyncPointer`) and non-threadsafe (`SharedPointer`) construct.

This is useful for when you need an API to *dynamically* be one of multiple
different pointer types. This has a slight overhead (virtual calls), so you
should not use this unless you know that's what you want.
