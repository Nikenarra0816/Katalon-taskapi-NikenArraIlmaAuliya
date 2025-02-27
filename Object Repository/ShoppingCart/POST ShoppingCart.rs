<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST ShoppingCart</name>
   <tag></tag>
   <elementGuidId>cacd6800-63f5-4bf4-966c-fc2e12d475ef</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;[\n  {\n    \&quot;book\&quot;: {\n      \&quot;bookId\&quot;: 2,\n      \&quot;title\&quot;: \&quot;Harry Potter and the Chamber of Secrets\&quot;,\n      \&quot;author\&quot;: \&quot;JKR\&quot;,\n      \&quot;category\&quot;: \&quot;Mystery\&quot;,\n      \&quot;price\&quot;: 236,\n      \&quot;coverFileName\&quot;: \&quot;9d8f4978-0ef8-42d0-873a-4eb583439237HP2.jpg\&quot;\n    },\n    \&quot;quantity\&quot;: 36\n  },\n  {\n    \&quot;book\&quot;: {\n      \&quot;bookId\&quot;: 6,\n      \&quot;title\&quot;: \&quot;Harry Potter and the Half-Blood Prince\&quot;,\n      \&quot;author\&quot;: \&quot;JKR\&quot;,\n      \&quot;category\&quot;: \&quot;Fantasy\&quot;,\n      \&quot;price\&quot;: 433,\n      \&quot;coverFileName\&quot;: \&quot;cff3d5ee-71f3-43d8-8625-33abcd48659eHP6.jpg\&quot;\n    },\n    \&quot;quantity\&quot;: 4\n  },\n  {\n    \&quot;book\&quot;: {\n      \&quot;bookId\&quot;: 7,\n      \&quot;title\&quot;: \&quot;Harry Potter and the Deathly Hallows\&quot;,\n      \&quot;author\&quot;: \&quot;JKR\&quot;,\n      \&quot;category\&quot;: \&quot;Fiction\&quot;,\n      \&quot;price\&quot;: 325,\n      \&quot;coverFileName\&quot;: \&quot;4ec2ffb6-b21a-43ce-bf90-04d56ec72644HP7.jpg\&quot;\n    },\n    \&quot;quantity\&quot;: 1\n  }\n]&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.auth_token}</value>
      <webElementGuid>355d001d-5060-4773-a312-497c59d20556</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>text/plain</value>
      <webElementGuid>ec214d99-0da9-4194-9389-c0192ea2e72a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>ee079c70-eae6-4b20-999b-ecc48042abf1</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://bookcart.azurewebsites.net/api/ShoppingCart/AddToCart/5200/2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
