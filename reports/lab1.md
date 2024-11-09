## 1:正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 三个 bad 测例 (ch2b_bad_*.rs) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

ch2b_bad_address.rs 尝试写入无效的内存 (0x0)
ch2b_bad_instructions.rs 尝试直接从S模式返回M模式，未进行环境的准备，报错
ch2b_bad_register.rs 尝试在用户态直接拿到寄存器sstatus的值



## 2：深入理解 trap.S 中两个函数 __alltraps 和 __restore 的作用，并回答如下问题:

### 2.1L40：刚进入 __restore 时，a0 代表了什么值。请指出 __restore 的两种使用情景。
__restore是trap后返回用户态的处理函数，因此此时a0的值应该是系统调用函数返回值或异常处理函数返回值。__restore在系统调用返回或异常处理返回时被调用。

### 2.2：L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

ld t0, 32*8(sp)
ld t1, 33*8(sp)
ld t2, 2*8(sp)
csrw sstatus, t0
csrw sepc, t1
csrw sscratch, t2

上述代码使用了三个CSR特权寄存器sstatus sepc和sscratch，在进入trap时，sstatus和sepc的值会被瞬间覆盖掉。其中sstatus用来存储和控制CPU的各种特权和状态信息，这里是恢复trap处理前存储的信息；sepc寄存器用来存储中断处下一条指令的位置，恢复sepc的值以从中断处继续执行；sscratch寄存器的值不会因为特权级切换等原因改变，用来保存一些临时数据，如中断上下文等，这里恢复的是用户栈位置；以上三个特权级寄存器只有在S mode才能使用。


## 3 L50-L56：为何跳过了 x2 和 x4？

ld x1, 1*8(sp)
ld x3, 3*8(sp)
.set n, 5
.rept 27
   LOAD_GP %n
   .set n, n+1
.endr

tp(x4)一般不会被用到，除非哦我们手动使用它，因此没有必要保存；而sp(x2)后面还要使用，我们需要依靠栈指针加偏移量来找到其他寄存器应该保存的正确位置。没有保存自然也没必要恢复。
## 4:L60：该指令之后，sp 和 sscratch 中的值分别有什么意义？
csrrw sp, sscratch, sp

这里是相当于交换了sscratch和sp的值，执行前sp指向内核栈指针，sscratch指向用户栈指针；这条指令执行后，sp就指向了用户栈，为回到用户态做准备。

## 5:__restore：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？
特权级切换发生在sret，这条指令用于从一个特权态的trap中返回。sret会把特权级恢复到sstatus寄存器的SPP字段存储的值，即上一个特权级，这样CPU就回到了用户态。sret还会把sstatus的SIE字段设置为SPIE( Supervisor Previous Interrupt Enable)字段，即恢复上一个中断使能位的状态，同时SPIE会被设置为1，SPP会被设置为支持的最低特权级（一般是User）。
## 6:L13：该指令之后，sp 和 sscratch 中的值分别有什么意义？

csrrw sp, sscratch, sp

和上一个问题一样，只不过结果相反而已
## 7:从 U 态进入 S 态是哪一条指令发生的？
ecall指令