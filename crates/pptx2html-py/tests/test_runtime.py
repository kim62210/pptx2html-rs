import tempfile
import unittest
import zipfile
from pathlib import Path

import pptx2html


class PythonBindingRuntimeTests(unittest.TestCase):
    def test_runtime_exports_match_public_stub_names(self) -> None:
        self.assertTrue(hasattr(pptx2html, "PresentationInfo"))
        self.assertTrue(hasattr(pptx2html, "ConversionResult"))
        self.assertTrue(hasattr(pptx2html, "UnresolvedElement"))

        with tempfile.TemporaryDirectory() as tmpdir:
            path = Path(tmpdir) / "sample.pptx"
            self._write_minimal_pptx(path)

            info = pptx2html.get_info(str(path))
            result = pptx2html.convert_with_metadata(str(path), slides=[1])

            self.assertEqual(type(info).__name__, "PresentationInfo")
            self.assertEqual(type(result).__name__, "ConversionResult")
            self.assertEqual(result.slide_count, 1)
            self.assertIsInstance(result.unresolved_elements, list)

    def test_missing_file_raises_runtime_error(self) -> None:
        with self.assertRaises(RuntimeError):
            pptx2html.convert_file("missing-file-does-not-exist.pptx")

    def _write_minimal_pptx(self, path: Path) -> None:
        with zipfile.ZipFile(path, "w") as archive:
            archive.writestr("[Content_Types].xml", self._content_types())
            archive.writestr("_rels/.rels", self._root_rels())
            archive.writestr("ppt/presentation.xml", self._presentation_xml())
            archive.writestr(
                "ppt/_rels/presentation.xml.rels",
                self._presentation_rels(),
            )
            archive.writestr("ppt/slides/slide1.xml", self._slide_xml("Slide One"))
            archive.writestr(
                "ppt/slides/_rels/slide1.xml.rels",
                self._empty_relationships(),
            )
            archive.writestr("ppt/theme/theme1.xml", self._theme_xml())

    def _content_types(self) -> str:
        return """<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>
<Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\">
  <Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>
  <Default Extension=\"xml\" ContentType=\"application/xml\"/>
  <Override PartName=\"/ppt/presentation.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml\"/>
  <Override PartName=\"/ppt/slides/slide1.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.slide+xml\"/>
  <Override PartName=\"/ppt/theme/theme1.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.theme+xml\"/>
</Types>"""

    def _root_rels(self) -> str:
        return """<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>
<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">
  <Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument\" Target=\"ppt/presentation.xml\"/>
</Relationships>"""

    def _presentation_xml(self) -> str:
        return """<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>
<p:presentation xmlns:a=\"http://schemas.openxmlformats.org/drawingml/2006/main\"
                xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\"
                xmlns:p=\"http://schemas.openxmlformats.org/presentationml/2006/main\">
  <p:sldMasterIdLst/>
  <p:sldIdLst>
    <p:sldId id=\"256\" r:id=\"rId1\"/>
  </p:sldIdLst>
  <p:sldSz cx=\"9144000\" cy=\"6858000\"/>
  <p:notesSz cx=\"6858000\" cy=\"9144000\"/>
</p:presentation>"""

    def _presentation_rels(self) -> str:
        return """<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>
<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">
  <Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide\" Target=\"slides/slide1.xml\"/>
  <Relationship Id=\"rId2\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme\" Target=\"theme/theme1.xml\"/>
</Relationships>"""

    def _slide_xml(self, text: str) -> str:
        return f"""<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>
<p:sld xmlns:a=\"http://schemas.openxmlformats.org/drawingml/2006/main\"
       xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\"
       xmlns:p=\"http://schemas.openxmlformats.org/presentationml/2006/main\">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id=\"1\" name=\"\"/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id=\"2\" name=\"TextBox 1\"/>
          <p:cNvSpPr txBox=\"1\"/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x=\"0\" y=\"0\"/><a:ext cx=\"914400\" cy=\"457200\"/></a:xfrm>
          <a:prstGeom prst=\"rect\"><a:avLst/></a:prstGeom>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p>
            <a:r>
              <a:rPr lang=\"en-US\" sz=\"1800\"/>
              <a:t>{text}</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
</p:sld>"""

    def _empty_relationships(self) -> str:
        return """<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>
<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\"/>"""

    def _theme_xml(self) -> str:
        return """<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>
<a:theme xmlns:a=\"http://schemas.openxmlformats.org/drawingml/2006/main\" name=\"TestTheme\">
  <a:themeElements>
    <a:clrScheme name=\"TestColors\">
      <a:dk1><a:srgbClr val=\"000000\"/></a:dk1>
      <a:lt1><a:srgbClr val=\"FFFFFF\"/></a:lt1>
      <a:dk2><a:srgbClr val=\"1F1F1F\"/></a:dk2>
      <a:lt2><a:srgbClr val=\"F7F7F7\"/></a:lt2>
      <a:accent1><a:srgbClr val=\"4472C4\"/></a:accent1>
      <a:accent2><a:srgbClr val=\"ED7D31\"/></a:accent2>
      <a:accent3><a:srgbClr val=\"A5A5A5\"/></a:accent3>
      <a:accent4><a:srgbClr val=\"FFC000\"/></a:accent4>
      <a:accent5><a:srgbClr val=\"5B9BD5\"/></a:accent5>
      <a:accent6><a:srgbClr val=\"70AD47\"/></a:accent6>
      <a:hlink><a:srgbClr val=\"0563C1\"/></a:hlink>
      <a:folHlink><a:srgbClr val=\"954F72\"/></a:folHlink>
    </a:clrScheme>
    <a:fontScheme name=\"TestFonts\">
      <a:majorFont><a:latin typeface=\"Calibri\"/></a:majorFont>
      <a:minorFont><a:latin typeface=\"Calibri\"/></a:minorFont>
    </a:fontScheme>
    <a:fmtScheme name=\"TestFmt\"/>
  </a:themeElements>
</a:theme>"""


if __name__ == "__main__":
    unittest.main()
