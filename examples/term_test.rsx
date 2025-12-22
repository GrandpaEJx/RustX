import "term" as term

term.clear()
print(term.bold(term.underline("RustX Terminal Module Test")))
print("")

print("Colors:")
print(term.red("  This is red"))
print(term.green("  This is green"))
print(term.blue("  This is blue"))
print(term.yellow("  This is yellow"))
print(term.cyan("  This is cyan"))
print(term.magenta("  This is magenta"))
print(term.white("  This is white"))

print("")
print("Styles:")
print(term.bold("  This is bold"))
print(term.dim("  This is dim"))
print(term.italic("  This is italic"))
print(term.underline("  This is underlined"))

print("")
print("Backgrounds:")
print(term.bg_red("  BG Red  "))
print(term.bg_green("  BG Green  "))
print(term.bg_blue("  BG Blue  "))
print(term.bg_yellow("  BG Yellow  "))
print(term.bg_cyan("  BG Cyan  "))
print(term.bg_magenta("  BG Magenta  "))
print(term.bg_white("  BG White  "))

print("")
print(term.green(term.bold("Test Complete!")))
