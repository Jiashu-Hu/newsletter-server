; ModuleID = 'autocfg_902bbb11b967177f_0.f1bfa8d0af85b34f-cgu.0'
source_filename = "autocfg_902bbb11b967177f_0.f1bfa8d0af85b34f-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

@__llvm_profile_runtime = external hidden global i32

; Function Attrs: noinline
define linkonce_odr hidden i32 @__llvm_profile_runtime_user() #0 {
  %1 = load i32, ptr @__llvm_profile_runtime, align 4
  ret i32 %1
}

attributes #0 = { noinline }

!llvm.module.flags = !{!0, !1, !2}
!llvm.ident = !{!3}
!llvm.dbg.cu = !{!4}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"Dwarf Version", i32 4}
!2 = !{i32 2, !"Debug Info Version", i32 3}
!3 = !{!"rustc version 1.83.0 (90b35a623 2024-11-26)"}
!4 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !5, producer: "clang LLVM (rustc version 1.83.0 (90b35a623 2024-11-26))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false, nameTableKind: None)
!5 = !DIFile(filename: "autocfg_902bbb11b967177f_0/@/autocfg_902bbb11b967177f_0.f1bfa8d0af85b34f-cgu.0", directory: "/Users/teddy/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slab-0.4.9")
