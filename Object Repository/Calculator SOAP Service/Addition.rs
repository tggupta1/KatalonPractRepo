<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Addition</name>
   <tag></tag>
   <elementGuidId>022dcd0a-1cb5-446f-818c-11f06be4f371</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soap:Envelope xmlns:soap=&quot;http://www.w3.org/2003/05/soap-envelope&quot; xmlns:tem=&quot;http://tempuri.org/&quot;>
   &lt;soap:Header/>
   &lt;soap:Body>
      &lt;tem:Add>
         &lt;tem:intA>7&lt;/tem:intA>
         &lt;tem:intB>3&lt;/tem:intB>
      &lt;/tem:Add>
   &lt;/soap:Body>
&lt;/soap:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP12</soapRequestMethod>
   <soapServiceEndpoint>http://www.dneonline.com/calculator.asmx</soapServiceEndpoint>
   <soapServiceFunction>Add</soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager
import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

print(response.responseText)
print(&quot;********TARUN 1***************&quot;)

WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)
print(&quot;********TARUN 2***************&quot;)

assertThat(response.getResponseText()).contains('10')
print(&quot;********TARUN 3***************&quot;)

//WS.verifyElementPropertyValue(response,'AddResponse.AddResult', '10')


</verificationScript>
   <wsdlAddress>http://www.dneonline.com/calculator.asmx?WSDL</wsdlAddress>
</WebServiceRequestEntity>
