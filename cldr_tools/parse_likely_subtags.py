import xml.etree.ElementTree as ET

tree = ET.parse('common/supplemental/likelySubtags.xml')
root = tree.getroot()
node = root.find('likelySubtags')

print('pub const LIKELY_SUBTAGS: &[(&str, &str)] = &[')
for child in node:
    key = child.attrib['from'].replace('_', '-')
    value = child.attrib['to'].replace('_', '-')
    print(f'    ("{key}", "{value}"),')
print('];')
