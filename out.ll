; ModuleID = 'test.ll'
source_filename = "test.ll"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

@msg = private unnamed_addr constant [23 x i8] c"Values: i64 %0, i64 %1\00", align 1

declare i32 @puts(i8*)

define i1 @main(i64 %0, i64 %1) {
entry:
  %load_value = load i64, i64* inttoptr (i64 12 to i64*), align 8
  %addvars = add i64 %load_value, %load_value
  %print_added_value = call i32 @puts(i64 %load_value)
  %compare = icmp eq i64 %0, %1
  %string = call i32 @puts(i8* getelementptr inbounds ([23 x i8], [23 x i8]* @msg, i32 0, i32 0))
  br i1 %compare, label %then, label %else

then:                                             ; preds = %entry
  ret i1 true

else:                                             ; preds = %entry
  ret i1 false
}
