<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>EditStudentData</name>
   <tag></tag>
   <elementGuidId>7755a43b-48b1-471e-924b-44282ca7863a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: ${ID},\n  \&quot;first_name\&quot;: \&quot;${FName}\&quot;,\n  \&quot;middle_name\&quot;: \&quot;sample string 3\&quot;,\n  \&quot;last_name\&quot;: \&quot;sample string 4\&quot;,\n  \&quot;date_of_birth\&quot;: \&quot;sample string 5\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>http://thetestingworldapi.com/Help/Api/PUT-api-studentsDetails-id/${ID}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'101'</defaultValue>
      <description></description>
      <id>fc22096a-11f5-4679-b2a0-ba0820947f55</id>
      <masked>false</masked>
      <name>ID</name>
   </variables>
   <variables>
      <defaultValue>'ABCD'</defaultValue>
      <description></description>
      <id>de9653d0-6ac6-4c55-b210-8b6ae3695d0d</id>
      <masked>false</masked>
      <name>FName</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
