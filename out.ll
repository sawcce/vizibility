; ModuleID = 'test.ll'
source_filename = "test.ll"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

@msg = private unnamed_addr constant [4 x i8] c"%i\0A\00", align 1
@msg.2 = private unnamed_addr constant [26 x i8] c"Arguments {a: %i, b: %i}\0A\00", align 1
@msg.3 = private unnamed_addr constant [56 x i8] c"Is \22%i\22 equal to \22%i\22 ? Answer: %i (0: true, 1: false)\0A\00", align 1

declare i32 @printf(i8*, ...)

define i1 @condition(i64 %0, i64 %1) {
entry:
  %load_value = load i64, i64* inttoptr (i64 12 to i64*), align 8
  %addvars = add i64 %load_value, %load_value
  %string = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @msg, i32 0, i32 0), i64 0)
  %compare = icmp eq i64 %0, %1
  %string1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([26 x i8], [26 x i8]* @msg.2, i32 0, i32 0), i64 %0, i64 %1)
  br i1 %compare, label %then, label %else

then:                                             ; preds = %entry
  ret i1 true

else:                                             ; preds = %entry
  ret i1 false
}

declare i32 @printf.1(i8*, ...)

define i64 @main() {
entry:
  %call = call i1 @condition(i64 25, i64 28)
  %string = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([56 x i8], [56 x i8]* @msg.3, i32 0, i32 0), i64 25, i64 28)
  ret i64 0
}
