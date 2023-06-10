import xml.etree.ElementTree as ET

tree = ET.parse('common/supplemental/likelySubtags.xml')
root = tree.getroot()
node = root.find('likelySubtags')

mappings = []
for child in node:
    key = child.attrib['from'].replace('_', '-')
    value = child.attrib['to'].replace('_', '-')
    mappings.append((key, value))
mappings.sort(key=lambda x: x[0])

print('pub const LIKELY_SUBTAGS: &[(&str, &str)] = &[')
for key, value in mappings:
    print(f'    ("{key}", "{value}"),')
print('];')
