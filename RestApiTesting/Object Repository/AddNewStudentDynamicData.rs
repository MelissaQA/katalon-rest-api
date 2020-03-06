<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddNewStudentDynamicData</name>
   <tag></tag>
   <elementGuidId>5058bda8-786b-49db-ab3d-94029240c842</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;first_name\&quot;: \&quot;${FirstName}\&quot;,\n  \&quot;middle_name\&quot;: \&quot;string\&quot;,\n  \&quot;last_name\&quot;: \&quot;${LastName}\&quot;,\n  \&quot;date_of_birth\&quot;: \&quot;${DOB}\&quot;\n}&quot;,
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
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://thetestingworldapi.com/Help/Api/POST-api-studentsDetails</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'A'</defaultValue>
      <description></description>
      <id>81b905bd-d96b-42a7-ad8a-2c14aefcc3f5</id>
      <masked>false</masked>
      <name>FirstName</name>
   </variables>
   <variables>
      <defaultValue>'B'</defaultValue>
      <description></description>
      <id>e213180d-f01c-4222-85d8-bf577b172f3b</id>
      <masked>false</masked>
      <name>LastName</name>
   </variables>
   <variables>
      <defaultValue>'C'</defaultValue>
      <description></description>
      <id>83817449-06a7-4292-bd95-035b62d46c56</id>
      <masked>false</masked>
      <name>DOB</name>
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
