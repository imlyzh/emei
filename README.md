# emei

The 峨眉 (EMei) JIT/AOT backend framework.

## Support Instructions

### x86_64

- mov
  - mov
  - mov_rev
  - movs(is movsq)
- push
  - push_reg
  - push_imm
  - push_all pusha/pushad

- add
  - add_to_eax
  - add_imm8
  - add_imm32
  - add

- ret
  - near_ret/near_ret_imm
  - far_ret/far_ret_imm

## Example

**lock src/main.rs**.
