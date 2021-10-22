# emei

The 峨眉 (EMei) JIT/AOT backend framework.

## Support Instructions

### x86_64

warning: ***unsupport fpu float operator***.

- mov
  - mov
  - mov_zero_extend_bit8/16
  - mov_sign_extend_bit8/16/32
  - mov_rev
  - movs(is movsq)
- push
  - push_reg
  - push_imm
  - push_all pusha/pushad

- add
  - add_first_reg
  - add_imm8
  - add_imm32
  - add
  - inc
  - inc_reg32

- sub
  - sub_first_reg
  - sub_signed_imm8
  - sub_imm(32)
  - sub
  - sub_rev
  - dec
  - dec_reg32

- jump
  - jmp
    - jmp to relative addr
    - jmp to addr literal (jit use it)
    - jmp to register
  - jump cond code
    - /ja/jb/jc/je/jg/jl/jo/jp/js/jz/jae/jbe/jge/jle/jpe/jpo/jna/jnb/jnc/jne/jng/jnl/jno/jnp/jns/jnz/jnae/jnbe/jnge/jnle

- call
  - call to relative addr
  - call to addr literal (jit use it)
  - call to register

- nop
  - nop(nop1)
  - nop1-9

- ret
  - near_ret/near_ret_imm
  - far_ret/far_ret_imm

#### sse float instruction

- movss/movsd
- addss/addsd
- subss/subsd
- mulss/mulsd
- divss/divsd
- sqrtss/sqrtsd
- cmpss/cmpsd

## Example

**look src/main.rs**.
