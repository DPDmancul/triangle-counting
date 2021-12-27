#!/usr/bin/env Rscript

df <- data.frame(read.csv("1_000_000.csv"))
colnames(df) <- c("arb_ord", "incidence")

print(nrow(df))
print(summary(df))
print(apply(df, 2 , sd))

par(mfrow=c(1,2))
hist(df$arb_ord, freq = FALSE)
hist(df$incidence, freq = FALSE)
