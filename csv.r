#!/usr/bin/env Rscript
args = commandArgs(trailingOnly=TRUE)

df <- data.frame(read.csv(args[1]))
colnames(df) <- c("arb_ord", "time", "incidence", "time")

print(nrow(df))
print(summary(df))
print(apply(df, 2 , sd))

par(mfrow=c(1,2))
hist(df$arb_ord, freq = FALSE)
hist(df$incidence, freq = FALSE)
