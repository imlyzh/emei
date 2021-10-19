# emei

The 峨眉 (EMei) JIT/AOT backend framework.

## Support Instructions

### x86_64

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
  - jump cond code
    - /ja/jb/jc/je/jg/jl/jo/jp/js/jz/jae/jbe/jge/jle/jpe/jpo/jna/jnb/jnc/jne/jng/jnl/jno/jnp/jns/jnz/jnae/jnbe/jnge/jnle

- nop
  - nop(nop1)
  - nop1-9

- ret
  - near_ret/near_ret_imm
  - far_ret/far_ret_imm

## Example

**lock src/main.rs**.
