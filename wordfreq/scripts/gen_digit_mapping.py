import unicodedata

print("const DIGIT_MAPPING: &[(char, u32)] = &[")
for i in range(0x110000):
    char = chr(i)
    if unicodedata.category(char) == 'Nd':
        print(f"    ('{char}', {int(char)}),")
print("];")
