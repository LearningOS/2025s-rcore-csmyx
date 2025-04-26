#### fstat:
##### questions:
    1. 如何根据fd文件描述符获取其对应的inode？
   
    fd_table中存的是`dyn File`的trait对象，目前其实际对象包括：stdin、stdout和OSInode。只有OSInode存在inode，stdin和stdout都没有inode，所以应该需要获取到该trait对象的原始对象。当原始对象不是OSInode时，返回-1。

    那么问题来了，如何将`trait object`转换成原始类型呢？貌似直接导入`core::any::Any`，然后就可以强制转换成`&dyn Any`就可以了。但是根据https://stackoverflow.com/questions/33687447/how-to-get-a-reference-to-a-concrete-type-from-a-trait-object。我们需要需要给File实现Any trait，同时还需要实现一个自定义的as_any方法。
    
    折腾到最后才发现，过于复杂了，既然反正都要修改File trait的定义了，为啥不直接在File trait里添加能否直接返回state的接口呢？仔细想了想，这才是正确的设计思路。因为，state是File的属性，就应该是在File里面存在公共的访问接口，然后各个具体的类型给出自己实现。不应该还需要使用者自己去downcasting到具体的类型才能获取到state。

    2. 但是我们只能通过OSInode获取到inode的内容，却无法获取到inode number，如何解决？

    我觉得最直接的方式是实现`get_diskinode_pos`的逆过程
