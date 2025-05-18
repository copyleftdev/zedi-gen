X12 835 Health Care Claim Payment/Advice (X221A1)
X12 Release 5010Revised May 3, 2024

Print
This X12 Transaction Set contains the format and establishes the data contents of the Health Care Claim Payment/Advice Transaction Set (835) for use within the context of the Electronic Data Interchange (EDI) environment.
This transaction set can be used to make a payment, send an Explanation of Benefits (EOB) remittance advice, or make a payment and send an EOB remittance advice only from a health insurer to a health care provider either directly or via a financial institution.

Delimiters
~ Segment
* Element
> Component
^ Repetition
EDI samples
Example 1: Dollars and Data Sent Separately
Example 2: Multiple Claims Single Check
Example 3: Claim Specific Negotiated Discount
Example 4: Claim Adjustment Reason Code 45
Example 5a: Line Service Tax impacting payment only
Example 5b: Line Service Bonuses impacting payment only
Example 5c: Line Service Penalty impacting payment only
Example 6: Not Covered/Not Authorized Inpatient Facility claim days
Example 8a: Claim submitted with incorrect subscriber as patient and incorrect ID
Example 8b: Claim submitted with incorrect subscriber name and ID
Example 8c: Claim submitted with for subscriber missing the Middle initial
ISA
Interchange Control Header
Required
Max use 1
To start and identify an interchange of zero or more functional groups and interchange-related control segments

Example
ISA*00*          *00*          *XX*XXXXXXXXXXXXXXX*XX*XXXXXXXXXXXXXXX*250519*2330*^*00501*000000000*X*X*>~
ISA-01
I01
Authorization Information Qualifier
Required
Identifier (ID)
Code identifying the type of information in the Authorization Information

00
No Authorization Information Present (No Meaningful Information in I02)
ISA-02
I02
Authorization Information
Required
String (AN)
Min 10
Max 10
Information used for additional identification or authorization of the interchange sender or the data in the interchange; the type of information is set by the Authorization Information Qualifier (I01)

ISA-03
I03
Security Information Qualifier
Required
Identifier (ID)
Code identifying the type of information in the Security Information

00
No Security Information Present (No Meaningful Information in I04)
ISA-04
I04
Security Information
Required
String (AN)
Min 10
Max 10
This is used for identifying the security information about the interchange sender or the data in the interchange; the type of information is set by the Security Information Qualifier (I03)

ISA-05
I05
Interchange ID Qualifier
Required
Identifier (ID)
Min 2
Max 2
Code indicating the system/method of code structure used to designate the sender or receiver ID element being qualified

Codes
ISA-06
I06
Interchange Sender ID
Required
String (AN)
Min 15
Max 15
Identification code published by the sender for other parties to use as the receiver ID to route data to them; the sender always codes this value in the sender ID element

ISA-07
I05
Interchange ID Qualifier
Required
Identifier (ID)
Min 2
Max 2
Code indicating the system/method of code structure used to designate the sender or receiver ID element being qualified

Codes
ISA-08
I07
Interchange Receiver ID
Required
String (AN)
Min 15
Max 15
Identification code published by the receiver of the data; When sending, it is used by the sender as their sending ID, thus other parties sending to them will use this as a receiving ID to route data to them

ISA-09
I08
Interchange Date
Required
Date (DT)
YYMMDD format
Date of the interchange

ISA-10
I09
Interchange Time
Required
Time (TM)
HHMM format
Time of the interchange

ISA-11
I65
Repetition Separator
Required
String (AN)
Min 1
Max 1
Type is not applicable; the repetition separator is a delimiter and not a data element; this field provides the delimiter used to separate repeated occurrences of a simple data element or a composite data structure; this value must be different than the data element separator, component element separator, and the segment terminator

^
Repetition Separator
ISA-12
I11
Interchange Control Version Number
Required
Identifier (ID)
Code specifying the version number of the interchange control segments

00501
Standards Approved for Publication by ASC X12 Procedures Review Board through October 2003
ISA-13
I12
Interchange Control Number
Required
Numeric (N0)
Min 9
Max 9
A control number assigned by the interchange sender

ISA-14
I13
Acknowledgment Requested
Required
Identifier (ID)
Min 1
Max 1
Code indicating sender's request for an interchange acknowledgment

0
No Interchange Acknowledgment Requested
1
Interchange Acknowledgment Requested (TA1)
ISA-15
I14
Interchange Usage Indicator
Required
Identifier (ID)
Min 1
Max 1
Code indicating whether data enclosed by this interchange envelope is test, production or information

I
Information
P
Production Data
T
Test Data
ISA-16
I15
Component Element Separator
Required
String (AN)
Min 1
Max 1
Type is not applicable; the component element separator is a delimiter and not a data element; this field provides the delimiter used to separate component data elements within a composite data structure; this value must be different than the data element separator and the segment terminator

>
Component Element Separator
GS
Functional Group Header
Required
Max use 1
To indicate the beginning of a functional group and to provide control information

Example
GS*HP*XXXXXXX*XXXXXXX*20250519*1443*000000000*X*005010X221A1~
GS-01
479
Functional Identifier Code
Required
Identifier (ID)
Code identifying a group of application related transaction sets

HP
Health Care Claim Payment/Advice (835)
GS-02
142
Application Sender's Code
Required
String (AN)
Min 2
Max 15
Code identifying party sending transmission; codes agreed to by trading partners

GS-03
124
Application Receiver's Code
Required
String (AN)
Min 2
Max 15
Code identifying party receiving transmission; codes agreed to by trading partners

GS-04
373
Date
Required
Date (DT)
CCYYMMDD format
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year

GS-05
337
Time
Required
Time (TM)
HHMM, HHMMSS, HHMMSSD, or HHMMSSDD format
Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)

GS-06
28
Group Control Number
Required
Numeric (N0)
Min 1
Max 9
Assigned number originated and maintained by the sender

GS-07
455
Responsible Agency Code
Required
Identifier (ID)
Min 1
Max 2
Code identifying the issuer of the standard; this code is used in conjunction with Data Element 480

T
Transportation Data Coordinating Committee (TDCC)
X
Accredited Standards Committee X12
GS-08
480
Version / Release / Industry Identifier Code
Required
String (AN)
Code indicating the version, release, subrelease, and industry identifier of the EDI standard being used, including the GS and GE segments; if code in DE455 in GS segment is X, then in DE 480 positions 1-3 are the version number; positions 4-6 are the release and subrelease, level of the version; and positions 7-12 are the industry or trade association identifiers (optionally assigned by user); if code in DE455 in GS segment is T, then other formats are allowed

005010X221A1
Heading
ST
0100
Transaction Set Header
Required
Max use 1
To indicate the start of a transaction set and to assign a control number

Example
ST*835*0001~
ST-01
143
Transaction Set Identifier Code
Required
Identifier (ID)
Code uniquely identifying a Transaction Set

The transaction set identifier (ST01) is used by the translation routines of the interchange partners to select the appropriate transaction set definition (e.g., 810 selects the Invoice Transaction Set).
Usage notes
The only valid value within this transaction set for ST01 is 835.
835
Health Care Claim Payment/Advice
ST-02
329
Transaction Set Control Number
Required
Numeric (N)
Min 4
Max 9
Identifying control number that must be unique within the transaction set functional group assigned by the originator for a transaction set

Usage notes
The Transaction Set Control Numbers in ST02 and SE02 must be identical. This unique number also aids in error resolution research. Start with a number, for example 0001, and increment from there. This number must be unique within a specific group and interchange, but it can be repeated in other groups and interchanges.
BPR
0200
Financial Information
Required
Max use 1
To indicate the beginning of a Payment Order/Remittance Advice Transaction Set and total payment amount, or to enable related transfer of funds and/or information from payer to payee to occur

Usage notes
Use the BPR to address a single payment to a single payee. A payee may represent a single provider, a provider group, or multiple providers in a chain. The BPR contains mandatory information, even when it is not being used to move funds electronically.
Example
BPR*U*0000000000000*D*FWT*CCP*04*XXXXXX*DA*X*XXXXXXXXXX*XXXXXXXXX*04*XXXX*DA*XXXXXX*20250519~
If either Depository Financial Institution (DFI) Identification Number Qualifier (BPR-06) or Sender DFI Identifier (BPR-07) is present, then the other is required
If Account Number Qualifier (BPR-08) is present, then Sender Bank Account Number (BPR-09) is required
If either Depository Financial Institution (DFI) Identification Number Qualifier (BPR-12) or Receiver or Provider Bank ID Number (BPR-13) is present, then the other is required
If Account Number Qualifier (BPR-14) is present, then Receiver or Provider Account Number (BPR-15) is required
BPR-01
305
Transaction Handling Code
Required
Identifier (ID)
Code designating the action to be taken by all parties

C
Payment Accompanies Remittance Advice
Use this code to instruct your third party processor to move both funds and remittance detail together through the banking system.

D
Make Payment Only
Use this code to instruct your third party processor to move only funds through the banking system and to ignore any remittance information.

H
Notification Only
Use this code when the actual provider payment (BPR02) is zero and the transaction is not being used for Prenotification of Future Transfers. This indicates remittance information without any associated payment.

I
Remittance Information Only
Use this code to indicate to the payee that the remittance detail is moving separately from the payment.

P
Prenotification of Future Transfers
This code is used only by the payer and the banking system to initially validate account numbers before beginning an EFT relationship. Contact your VAB for additional information.

U
Split Payment and Remittance
Use this code to instruct the third party processor to split the payment and remittance detail, and send each on a separate path.

X
Handling Party's Option to Split Payment and Remittance
Use this code to instruct the third party processor to move the payment and remittance detail, either together or separately, based upon end point requests or capabilities.

BPR-02
782
Total Actual Provider Payment Amount
Required
Decimal number (R)
Min 1
Max 15
Monetary amount

BPR02 specifies the payment amount.
Usage notes
Use BPR02 for the total payment amount for this 835. The total payment amount for this 835 cannot exceed eleven characters, including decimals (99999999.99). Although the value can be zero, the 835 cannot be issued for less than zero dollars.
Decimal elements will be limited to a maximum length of 10 characters including reported or implied places for cents (implied value of 00 after the decimal point).
BPR-03
478
Credit or Debit Flag Code
Required
Identifier (ID)
Code indicating whether amount is a credit or debit

C
Credit
Use this code to indicate a credit to the provider's account and a debit to the payer's account, initiated by the payer. In the case of an EFT, no additional action is required of the provider. Also use this code when a check is issued for the payment.

D
Debit
Use this code to indicate a debit to the payer's account and a credit to the provider's account, initiated by the provider at the instruction of the payer. Extreme caution must be used when using Debit transactions. Contact your VAB for information about debit transactions. The rest of this segment and document assumes that a credit payment is being used.

BPR-04
591
Payment Method Code
Required
Identifier (ID)
Code identifying the method for the movement of payment instructions

ACH
Automated Clearing House (ACH)
Use this code to move money electronically through the ACH, or to notify the provider that an ACH transfer was requested. When this code is used, see BPR05 through BPR15 for additional requirements.

BOP
Financial Institution Option
Use this code to indicate that the third party processor will choose the method of payment based upon end point requests or capabilities. When this code is used, see BPR05 through BPR15 for additional requirements.

CHK
Check
Use this code to indicate that a check has been issued for payment.

FWT
Federal Reserve Funds/Wire Transfer - Nonrepetitive
Use this code to indicate that the funds were sent through the wire system. When this code is used, see BPR05 through BPR15 for additional requirements.

NON
Non-Payment Data
Use this code when the Transaction Handling Code (BPR01) is H, indicating that this is information only and no dollars are to be moved.

BPR-05
812
Payment Format Code
Optional
Identifier (ID)
Code identifying the payment format to be used

CCP
Cash Concentration/Disbursement plus Addenda (CCD+) (ACH)
Use the CCD+ format to move money and up to 80 characters of data, enough to reassociate dollars and data when the dollars are sent through the ACH and the data is sent on a separate path. The addenda must contain a copy of the TRN segment.

CTX
Corporate Trade Exchange (CTX) (ACH)
Use the CTX format to move dollars and data through the ACH. The CTX format can contain up to 9,999 addenda records of 80 characters each. The CTX encapsulates the complete 835 and all envelope segments.

BPR-06
506
Depository Financial Institution (DFI) Identification Number Qualifier
Optional
Identifier (ID)
Code identifying the type of identification number of Depository Financial Institution (DFI)

When using this transaction set to initiate a payment, all or some of BPR06 through BPR16 may be required, depending on the conventions of the specific financial channel being used.
BPR06 and BPR07 relate to the originating depository financial institution (ODFI).
Usage notes
BPR06 through BPR09 relate to the originating financial institution and the originator's account (payer).
01
ABA Transit Routing Number Including Check Digits (9 digits)
The ABA transit routing number is a unique number identifying every bank in the United States.

04
Canadian Bank Branch and Institution Number
BPR-07
507
Sender DFI Identifier
Optional
String (AN)
Min 3
Max 12
Depository Financial Institution (DFI) identification number

Usage notes
Use this number for the identifying number of the financial institution sending the transaction into the applicable network.
BPR-08
569
Account Number Qualifier
Optional
Identifier (ID)
Code indicating the type of account

BPR08 is a code identifying the type of bank account or other financial asset.
Usage notes
Use this code to identify the type of account in BPR09.
DA
Demand Deposit
BPR-09
508
Sender Bank Account Number
Optional
String (AN)
Min 1
Max 35
Account number assigned

BPR09 is the account of the company originating the payment. This account may be debited or credited depending on the type of payment order.
Usage notes
Use this number for the originator's account number at the financial institution.
BPR-10
509
Payer Identifier
Optional
String (AN)
Min 10
Max 10
A unique identifier designating the company initiating the funds transfer instructions, business transaction or assigning tracking reference identification.

BPR10 shall be mutually established between the originating depository financial institution (ODFI) and the company originating the payment.
BPR-11
510
Originating Company Supplemental Code
Optional
String (AN)
Min 9
Max 9
A code defined between the originating company and the originating depository financial institution (ODFI) that uniquely identifies the company initiating the transfer instructions

Usage notes
Use this code to further identify the payer by division or region. The element must be left justified and space filled to meet the minimum element size requirements. If used, this code must be identical to TRN04, excluding trailing spaces.
BPR-12
506
Depository Financial Institution (DFI) Identification Number Qualifier
Optional
Identifier (ID)
Code identifying the type of identification number of Depository Financial Institution (DFI)

BPR12 and BPR13 relate to the receiving depository financial institution (RDFI).
Usage notes
BPR12 through BPR15 relate to the receiving financial institution and the receiver's account.
01
ABA Transit Routing Number Including Check Digits (9 digits)
The ABA transit routing number is a unique number identifying every bank in the United States.

04
Canadian Bank Branch and Institution Number
BPR-13
507
Receiver or Provider Bank ID Number
Optional
String (AN)
Min 3
Max 12
Depository Financial Institution (DFI) identification number

Usage notes
Use this number for the identifying number of the financial institution receiving the transaction from the applicable network.
BPR-14
569
Account Number Qualifier
Optional
Identifier (ID)
Code indicating the type of account

BPR14 is a code identifying the type of bank account or other financial asset.
Usage notes
Use this code to identify the type of account in BPR15.
DA
Demand Deposit
SG
Savings
BPR-15
508
Receiver or Provider Account Number
Optional
String (AN)
Min 1
Max 35
Account number assigned

BPR15 is the account number of the receiving company to be debited or credited with the payment order.
Usage notes
Use this number for the receiver's account number at the financial institution.
BPR-16
373
Check Issue or EFT Effective Date
Required
Date (DT)
CCYYMMDD format
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year

BPR16 is the date the originating company intends for the transaction to be settled (i.e., Payment Effective Date).
Usage notes
Use this for the effective entry date. If BPR04 is ACH, this is the date that the money moves from the payer and is available to the payee. If BPR04 is CHK, this is the check issuance date. If BPR04 is FWT, this is the date that the payer anticipates the money to move. As long as the effective date is a business day, this is the settlement date. If BPR04 is `NON', enter the date of the 835.
TRN
0400
Reassociation Trace Number
Required
Max use 1
To uniquely identify a transaction to an application

Usage notes
This segment's purpose is to uniquely identify this transaction set and to aid in reassociating payments and remittances that have been separated.
Example
TRN*1*XXX*XXXXXXXXXX*XXXXXX~
TRN-01
481
Trace Type Code
Required
Identifier (ID)
Code identifying which transaction is being referenced

1
Current Transaction Trace Numbers
TRN-02
127
Check or EFT Trace Number
Required
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

TRN02 provides unique identification for the transaction.
Usage notes
This number must be unique within the sender/receiver relationship. The number is assigned by the sender. If payment is made by check, this must be the check number. If payment is made by EFT, this must be the EFT reference number. If this is a non-payment 835, this must be a unique remittance advice identification number.
See 1.10.2.3, Reassociation of Dollars and Data, for additional information.
TRN-03
509
Payer Identifier
Required
String (AN)
Min 10
Max 10
A unique identifier designating the company initiating the funds transfer instructions, business transaction or assigning tracking reference identification.

TRN03 identifies an organization.
Usage notes
This must be a 1 followed by the payer's EIN (or TIN).
TRN-04
127
Originating Company Supplemental Code
Optional
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

TRN04 identifies a further subdivision within the organization.
Usage notes
If both TRN04 and BPR11 are used, they must be identical, excluding trailing spaces. Since BPR11 has a min/max value of 9/9, whenever both are used, this element is restricted to a maximum size of 9.
CUR
0500
Foreign Currency Information
Optional
Max use 1
To specify the currency (dollars, pounds, francs, etc.) used in a transaction

Usage notes
When the CUR segment is not present, the currency of payment is defined as US dollars.
Required when the payment is not being made in US dollars. If not required by this implementation guide, do not send.
Example
CUR*PR*XXX~
CUR-01
98
Entity Identifier Code
Required
Identifier (ID)
Code identifying an organizational entity, a physical location, property or an individual

PR
Payer
CUR-02
100
Currency Code
Required
Identifier (ID)
Min 3
Max 3
Code (Standard ISO) for country in whose currency the charges are specified

Usage notes
This is the currency code for the payment currency.
REF
0600
Receiver Identification
Optional
Max use 1
To specify identifying information

Usage notes
This is the business identification information for the transaction receiver. This may be different than the EDI address or identifier of the receiver. This is the initial receiver of the transaction. This information must not be updated if the transaction is routed through multiple intermediaries, such as clearinghouses, before reaching the payee.
Required when the receiver of the transaction is other than the payee (e.g., a clearinghouse or billing service). If not required by this implementation guide, may be provided at sender's discretion, but cannot be required by the receiver.
Example
REF*EV*XXXXX~
Variants (all may be used)
REFVersion Identification
REF-01
128
Reference Identification Qualifier
Required
Identifier (ID)
Code qualifying the Reference Identification

EV
Receiver Identification Number
REF-02
127
Receiver Identifier
Required
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

REF
0600
Version Identification
Optional
Max use 1
To specify identifying information

Usage notes
Update this reference number whenever a change in the version or release number affects the 835. (This is not the ANSI ASCX12 version number as reported in the GS segment.)
Required when necessary to report the version number of the adjudication system that generated the claim payments in order for the payer to resolve customer service questions from the payee. If not required by this implementation guide, do not send.
Example
REF*F2*XXXX~
Variants (all may be used)
REFReceiver Identification
REF-01
128
Reference Identification Qualifier
Required
Identifier (ID)
Code qualifying the Reference Identification

F2
Version Code - Local
REF-02
127
Version Identification Code
Required
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

DTM
0700
Production Date
Optional
Max use 1
To specify pertinent dates and times

Usage notes
If your adjudication cycle completed on Thursday and your 835 is produced on Saturday, you are required to populate this segment with Thursday's date.
Required when the cut off date of the adjudication system remittance run is different from the date of the 835 as identified in the related GS04 element. If not required by this implementation guide, may be provided at the sender's discretion, but cannot be required by the receiver.
Example
DTM*405*20250519~
DTM-01
374
Date Time Qualifier
Required
Identifier (ID)
Code specifying type of date or time, or both date and time

405
Production
DTM-02
373
Production Date
Required
Date (DT)
CCYYMMDD format
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year

Usage notes
Report the end date for the adjudication production cycle for claims included in this 835.
1000A Payer Identification Loop
Required
Max 1
Variants (all may be used)
Payee Identification Loop
N1
0800
Payer Identification
Required
Max use 1
To identify a party by type of organization, name, and code

Usage notes
Use this N1 loop to provide the name/address information for the payer.
The payer's secondary identifying reference number is provided in N104, if necessary.
Example
N1*PR*XXXX*XV*XXXXXXX~
If either Identification Code Qualifier (N1-03) or Payer Identifier (N1-04) is present, then the other is required
N1-01
98
Entity Identifier Code
Required
Identifier (ID)
Code identifying an organizational entity, a physical location, property or an individual

PR
Payer
N1-02
93
Payer Name
Required
String (AN)
Min 1
Max 60
Free-form name

N1-03
66
Identification Code Qualifier
Optional
Identifier (ID)
Code designating the system/method of code structure used for Identification Code (67)

XV
Centers for Medicare and Medicaid Services PlanID
Use when reporting Health Plan ID (HPID) or Other Entity Identifier (OEID).

N1-04
67
Payer Identifier
Optional
String (AN)
Min 2
Max 80
Code identifying a party or other code

This segment, used alone, provides the most efficient method of providing organizational identification. To obtain this efficiency the "ID Code" (N104) must provide a key to the table maintained by the transaction processing party.
N3
1000
Payer Address
Required
Max use 1
To specify the location of the named party

Example
N3*XXX*XXXX~
N3-01
166
Payer Address Line
Required
String (AN)
Min 1
Max 55
Address information

N3-02
166
Payer Address Line
Optional
String (AN)
Min 1
Max 55
Address information

N4
1100
Payer City, State, ZIP Code
Required
Max use 1
To specify the geographic place of the named party

Example
N4*XXXXXX*XX*XXXXXX*XXX~
Only one of Payer State Code (N4-02) or Country Subdivision Code (N4-07) may be present
If Country Subdivision Code (N4-07) is present, then Country Code (N4-04) is required
N4-01
19
Payer City Name
Required
String (AN)
Min 2
Max 30
Free-form text for city name

A combination of either N401 through N404, or N405 and N406 may be adequate to specify a location.
N4-02
156
Payer State Code
Optional
Identifier (ID)
Min 2
Max 2
Code (Standard State/Province) as defined by appropriate government agency

N402 is required only if city name (N401) is in the U.S. or Canada.
N4-03
116
Payer Postal Zone or ZIP Code
Optional
Identifier (ID)
Min 3
Max 15
Code defining international postal zone code excluding punctuation and blanks (zip code for United States)

N4-04
26
Country Code
Optional
Identifier (ID)
Min 2
Max 3
Code identifying the country

Usage notes
Use the alpha-2 country codes from Part 1 of ISO 3166.
N4-07
1715
Country Subdivision Code
Optional
Identifier (ID)
Min 1
Max 3
Code identifying the country subdivision

Usage notes
Use the country subdivision codes from Part 2 of ISO 3166.
REF
1200
Additional Payer Identification
Optional
Max use 4
To specify identifying information

Usage notes
The ID available in the TRN and N1 segments must be used before using the REF segment.
Required when additional payer identification numbers beyond those in the TRN and Payer N1 segments are needed. If not required by this implementation guide, may be sent at sender's discretion, but cannot be required by the receiver.
Example
REF*HI*XX~
REF-01
128
Reference Identification Qualifier
Required
Identifier (ID)
Code qualifying the Reference Identification

2U
Payer Identification Number
For Medicare carriers or intermediaries, use this qualifier for the Medicare carrier or intermediary ID number. For Blue Cross and Blue Shield Plans, use this qualifier for the Blue Cross Blue Shield association plan code.

EO
Submitter Identification Number
This is required when the original transaction sender is not the payer or is identified by an identifier other than those already provided. This is not updated by third parties between the payer and the payee. An example of a use for this qualifier is when identifying a clearinghouse that created the 835 when the health plan sent a proprietary format to the clearinghouse.

HI
Health Industry Number (HIN)
NF
National Association of Insurance Commissioners (NAIC) Code
This is the preferred value when identifying the payer.

REF-02
127
Additional Payer Identifier
Required
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

PER
1300
Payer Business Contact Information
Optional
Max use 1
To identify a person or office to whom administrative communications should be directed

Usage notes
When the communication number represents a telephone number in the United States and other countries using the North American Dialing Plan (for voice, data, fax, etc.), the communication number always includes the area code and phone number using the format AAABBBCCCC. Where AAA is the area code, BBB is the telephone number prefix, and CCCC is the telephone number (e.g. (800) 555-1212 would be represented as 8005551212). The extension number, when applicable, is identified in the next element pair (Communications Number Qualifier and Communication Number) immediately after the telephone number.
Required when there is a business contact area that would apply to this remittance and all the claims. If not required by this implementation guide, do not send.
Example
PER*CX*XXXXXX*FX*XXXX*EM*X*EX*XX~
Variants (all may be used)
PERPayer Technical Contact Information
PERPayer WEB Site
If either Communication Number Qualifier (PER-03) or Payer Contact Communication Number (PER-04) is present, then the other is required
If either Communication Number Qualifier (PER-05) or Payer Contact Communication Number (PER-06) is present, then the other is required
If either Communication Number Qualifier (PER-07) or Payer Contact Communication Number (PER-08) is present, then the other is required
PER-01
366
Contact Function Code
Required
Identifier (ID)
Code identifying the major duty or responsibility of the person or group named

CX
Payers Claim Office
PER-02
93
Payer Contact Name
Optional
String (AN)
Min 1
Max 60
Free-form name

Usage notes
Use this data element when the name of the individual to contact is not already defined or is different than the name within the prior name segment (e.g. N1 or NM1).
PER-03
365
Communication Number Qualifier
Optional
Identifier (ID)
Code identifying the type of communication number

EM
Electronic Mail
FX
Facsimile
TE
Telephone
PER-04
364
Payer Contact Communication Number
Optional
String (AN)
Min 1
Max 256
Complete communications number including country or area code when applicable

PER-05
365
Communication Number Qualifier
Optional
Identifier (ID)
Code identifying the type of communication number

EM
Electronic Mail
EX
Telephone Extension
When used, the value following this code is the extension for the preceding communications contact number.

FX
Facsimile
TE
Telephone
PER-06
364
Payer Contact Communication Number
Optional
String (AN)
Min 1
Max 256
Complete communications number including country or area code when applicable

PER-07
365
Communication Number Qualifier
Optional
Identifier (ID)
Code identifying the type of communication number

EX
Telephone Extension
PER-08
364
Payer Contact Communication Number
Optional
String (AN)
Min 1
Max 256
Complete communications number including country or area code when applicable

PER
1300
Payer Technical Contact Information
Required
Max use >1
To identify a person or office to whom administrative communications should be directed

Usage notes
Required to report technical contact information for this remittance advice.
Example
PER*BL*XXX*EM*XX*EM*XXX*UR*XXXX~
Variants (all may be used)
PERPayer Business Contact Information
PERPayer WEB Site
If either Communication Number Qualifier (PER-03) or Payer Contact Communication Number (PER-04) is present, then the other is required
If either Communication Number Qualifier (PER-05) or Payer Technical Contact Communication Number (PER-06) is present, then the other is required
If either Communication Number Qualifier (PER-07) or Payer Contact Communication Number (PER-08) is present, then the other is required
PER-01
366
Contact Function Code
Required
Identifier (ID)
Code identifying the major duty or responsibility of the person or group named

BL
Technical Department
PER-02
93
Payer Technical Contact Name
Optional
String (AN)
Min 1
Max 60
Free-form name

Usage notes
Use this data element when the name of the individual to contact is not already defined or is different than the name within the prior name segment (e.g. N1 or NM1).
PER-03
365
Communication Number Qualifier
Optional
Identifier (ID)
Code identifying the type of communication number

EM
Electronic Mail
TE
Telephone
Recommended

UR
Uniform Resource Locator (URL)
Use only when there is no central telephone number for the payer entity.

PER-04
364
Payer Contact Communication Number
Optional
String (AN)
Min 1
Max 256
Complete communications number including country or area code when applicable

PER-05
365
Communication Number Qualifier
Optional
Identifier (ID)
Code identifying the type of communication number

EM
Electronic Mail
EX
Telephone Extension
When used, the value following this code is theextension for the preceding communicationscontact number.

FX
Facsimile
TE
Telephone
UR
Uniform Resource Locator (URL)
PER-06
364
Payer Technical Contact Communication Number
Optional
String (AN)
Min 1
Max 256
Complete communications number including country or area code when applicable

PER-07
365
Communication Number Qualifier
Optional
Identifier (ID)
Code identifying the type of communication number

EM
Electronic Mail
EX
Telephone Extension
When used, the value following this code is theextension for the preceding communicationscontact number.

FX
Facsimile
UR
Uniform Resource Locator (URL)
PER-08
364
Payer Contact Communication Number
Optional
String (AN)
Min 1
Max 256
Complete communications number including country or area code when applicable

PER
1300
Payer WEB Site
Optional
Max use 1
To identify a person or office to whom administrative communications should be directed

Usage notes
Required when any 2110 loop Healthcare Policy REF Segment is used. If not required by this implementation guide, do not send.
This is a direct link to the policy location of the un-secure website.
Example
PER*IC**UR*XXXX~
Variants (all may be used)
PERPayer Business Contact Information
PERPayer Technical Contact Information
PER-01
366
Contact Function Code
Required
Identifier (ID)
Code identifying the major duty or responsibility of the person or group named

IC
Information Contact
PER-03
365
Communication Number Qualifier
Required
Identifier (ID)
Code identifying the type of communication number

UR
Uniform Resource Locator (URL)
PER-04
364
Communication Number
Required
String (AN)
Min 1
Max 256
Complete communications number including country or area code when applicable

Usage notes
This is the payer's WEB site URL where providers can find policy and other related information.
1000B Payee Identification Loop
Required
Max 1
Variants (all may be used)
Payer Identification Loop
N1
0800
Payee Identification
Required
Max use 1
To identify a party by type of organization, name, and code

Usage notes
Use this N1 loop to provide the name/address information of the payee. The identifying reference number is provided in N104.
Example
N1*PE*XXX*FI*XXXXXXX~
N1-01
98
Entity Identifier Code
Required
Identifier (ID)
Code identifying an organizational entity, a physical location, property or an individual

PE
Payee
N1-02
93
Payee Name
Required
String (AN)
Min 1
Max 60
Free-form name

N1-03
66
Identification Code Qualifier
Required
Identifier (ID)
Code designating the system/method of code structure used for Identification Code (67)

FI
Federal Taxpayer's Identification Number
Required if provider is not mandated by NPI. For individual providers as payees, use this qualifier to represent the Social Security Number.

XV
Centers for Medicare and Medicaid Services PlanID
Use when reporting Health Plan ID (HPID) or Other Entity Identifier (OEID). This only applies in cases of post payment recovery. See section 1.10.2.16 (Post Payment Recovery) for further information.

XX
Centers for Medicare and Medicaid Services National Provider Identifier
This is REQUIRED when the National Provider Identifier is mandated for use and the payee is a covered health care provider under the mandate.

N1-04
67
Payee Identification Code
Required
String (AN)
Min 2
Max 80
Code identifying a party or other code

This segment, used alone, provides the most efficient method of providing organizational identification. To obtain this efficiency the "ID Code" (N104) must provide a key to the table maintained by the transaction processing party.
N3
1000
Payee Address
Optional
Max use 1
To specify the location of the named party

Usage notes
Required when the sender needs to communicate the payee address to a transaction receiver, e.g., a VAN or a clearinghouse. If not required by this implementation guide, may be provided at the sender's discretion, but cannot be required by the receiver.
Example
N3*X*XXXXX~
N3-01
166
Payee Address Line
Required
String (AN)
Min 1
Max 55
Address information

N3-02
166
Payee Address Line
Optional
String (AN)
Min 1
Max 55
Address information

N4
1100
Payee City, State, ZIP Code
Optional
Max use 1
To specify the geographic place of the named party

Usage notes
Required when the sender needs to communicate the payee address to a transaction receiver, e.g., a VAN or a clearinghouse. If not required by this implementation guide, may be provided at the sender's discretion, but cannot be required by the receiver.
Example
N4*XXXXXX*XX*XXXXX*XXX~
Only one of Payee State Code (N4-02) or Country Subdivision Code (N4-07) may be present
If Country Subdivision Code (N4-07) is present, then Country Code (N4-04) is required
N4-01
19
Payee City Name
Required
String (AN)
Min 2
Max 30
Free-form text for city name

A combination of either N401 through N404, or N405 and N406 may be adequate to specify a location.
N4-02
156
Payee State Code
Optional
Identifier (ID)
Min 2
Max 2
Code (Standard State/Province) as defined by appropriate government agency

N402 is required only if city name (N401) is in the U.S. or Canada.
N4-03
116
Payee Postal Zone or ZIP Code
Optional
Identifier (ID)
Min 3
Max 15
Code defining international postal zone code excluding punctuation and blanks (zip code for United States)

N4-04
26
Country Code
Optional
Identifier (ID)
Min 2
Max 3
Code identifying the country

Usage notes
Use the alpha-2 country codes from Part 1 of ISO 3166.
N4-07
1715
Country Subdivision Code
Optional
Identifier (ID)
Min 1
Max 3
Code identifying the country subdivision

Usage notes
Use the country subdivision codes from Part 2 of ISO 3166.
REF
1200
Payee Additional Identification
Optional
Max use >1
To specify identifying information

Usage notes
Required when identification of the payee is dependent upon an identification number beyond that supplied in the N1 segment. If not required by this implementation guide, may be provided at the sender's discretion, but cannot be required by the receiver.
Example
REF*0B*XXXXXX~
REF-01
128
Reference Identification Qualifier
Required
Identifier (ID)
Code qualifying the Reference Identification

0B
State License Number
D3
National Council for Prescription Drug Programs Pharmacy Number
PQ
Payee Identification
TJ
Federal Taxpayer's Identification Number
This information must be in the N1 segment unless the National Provider ID or the Health Plan Identifier (HPID) or Other Entity Identifier (OEID) was used in N104. For individual providers as payees, use this number to represent the Social Security Number. TJ also represents the Employer Identification Number (EIN). According to the IRS, TIN and EIN can be used interchangeably.

REF-02
127
Additional Payee Identifier
Required
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

RDM
1400
Remittance Delivery Method
Optional
Max use 1
To identify remittance delivery when remittance is separate from payment

Usage notes
Required when BPR01 = U or X; and the remittance is to be sent separately from the payment. The payer is responsible to provide the bank with the instructions on how to deliver the remittance information, if not required by this implementation guide, do not send.
Payer should coordinate this process with their Originating Depository Financial Institution (ODFI).
Example
RDM*FT*XXXXXX*XXX~
RDM-01
756
Report Transmission Code
Required
Identifier (ID)
Code defining timing, transmission method or format by which reports are to be sent

BM
By Mail
When used, RDM02 must be used.

When BM is used, the remittance information will be mailed to the payee at the address identified in this 1000B loop.

EM
E-Mail
Use with encrypted e-mail.

FT
File Transfer
Use with FTP communications.

OL
On-Line
Use with secured hosted or other electronic delivery.

RDM-02
93
Name
Optional
String (AN)
Min 1
Max 60
Free-form name

RDM02 is used to contain the name of a third party processor if needed, who would be the first recipient of the remittance.
Usage notes
When BM is used, the remittance information will be mailed to the attention of this person at the payee's address identified in this 1000B loop.
RDM-03
364
Communication Number
Optional
String (AN)
Min 1
Max 256
Complete communications number including country or area code when applicable

RDM03 contains the operative communication number for the delivery method specified in RDM01 (i.e. fax phone number and mail address).
Usage notes
Contains URL web address or e-mail address.
Detail
2000 Header Number Loop
Optional
Max >1
LX
0030
Header Number
Required
Max use 1
To reference a line number in a transaction set

Usage notes
Required when claim/service information is being provided in the transaction. If not required by this implementation guide, do not send.
The purpose of LX01 is to provide an identification of a particular grouping of claims for sorting purposes.
In the event that claim/service information must be sorted, the LX segment must precede each series of claim level and service level segments. This number is intended to be unique within each transaction.
Example
LX*000000~
LX-01
554
Assigned Number
Required
Numeric (N0)
Min 1
Max 6
Number assigned for differentiation within a transaction set

TS3
0050
Provider Summary Information
Optional
Max use 1
To supply provider-level control information

Usage notes
TS301 identifies the subsidiary provider.
The remaining mandatory elements (TS302 through TS305) must be valid with appropriate data, as defined by the TS3 segment.
Only Medicare Part A uses data elements TS313, TS315, TS317, TS318 and TS320 through TS324. Each monetary amount element is for that provider for this facility type code for loop 2000.
Required for Medicare Part A or when payers and payees outside the Medicare Part A community need to identify provider subsidiaries whose remittance information is contained in the 835 transactions transmitted to a single provider entity [i.e., the corporate office of a hospital chain]. If not required by this implementation guide, do not send.
Example
TS3*XX*X*20250519*000000000000000*000000000000********00000000**000000**000000000000*00000000000**0000000000000*000000000000*000000000*000000000000*0000000000~
TS3-01
127
Provider Identifier
Required
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

TS301 is the provider number.
Usage notes
This is the provider number.
TS3-02
1331
Facility Type Code
Required
String (AN)
Min 1
Max 2
Code identifying where services were, or may be, performed; the first and second positions of the Uniform Bill Type Code for Institutional Services or the Place of Service Codes for Professional or Dental Services.

Usage notes
When reporting a TS3 segment for professional claims and the claims are not all for the same place of service, report a place of service of 11 (Office) as the default value. When reporting a TS3 segment for pharmaceutical claims and the claims are not all for the same place of service, report a place of service of 99 (Other unlisted facility) as the default value.
TS3-03
373
Fiscal Period Date
Required
Date (DT)
CCYYMMDD format
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year

TS303 is the last day of the provider's fiscal year.
Usage notes
Use this date for the last day of the provider's fiscal year. If the end of the provider's fiscal year is not known, use December 31st of the current year.
TS3-04
380
Total Claim Count
Required
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

TS304 is the total number of claims.
Usage notes
This is the total number of claims.
TS3-05
782
Total Claim Charge Amount
Required
Decimal number (R)
Min 1
Max 15
Monetary amount

TS305 is the total of reported charges.
Usage notes
This is the total reported charges for all claims.
Decimal elements will be limited to a maximum length of 10 characters including reported or implied places for cents (implied value of 00 after the decimal point). This applies to all 782 elements.
TS3-13
782
Total MSP Payer Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS313 is the total Medicare Secondary Payer (MSP) primary payer amount.
Usage notes
See TR3 note 3.
TS3-15
782
Total Non-Lab Charge Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS315 is the summary of non-lab charges.
Usage notes
See TR3 note 3.
TS3-17
782
Total HCPCS Reported Charge Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS317 is the Health Care Financing Administration Common Procedural Coding System (HCPCS) reported charges.
Usage notes
See TR3 note 3.
TS3-18
782
Total HCPCS Payable Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS318 is the total Health Care Financing Administration Common Procedural Coding System (HCPCS) payable amount.
Usage notes
See TR3 note 3.
TS3-20
782
Total Professional Component Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS320 is the total professional component amount.
Usage notes
The professional component amount must also be reported in the CAS segment with a Claim Adjustment Reason Code value of 89.
See TR3 note 3.
TS3-21
782
Total MSP Patient Liability Met Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS321 is the total Medicare Secondary Payer (MSP) patient liability met.
Usage notes
See TR3 note 3.
TS3-22
782
Total Patient Reimbursement Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS322 is the total patient reimbursement.
Usage notes
See TR3 note 3.
TS3-23
380
Total PIP Claim Count
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

TS323 is the total periodic interim payment (PIP) number of claims.
Usage notes
See TR3 note 3.
TS3-24
782
Total PIP Adjustment Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS324 is total periodic interim payment (PIP) adjustment.
Usage notes
See TR3 note 3.
TS2
0070
Provider Supplemental Summary Information
Optional
Max use 1
To provide supplemental summary control information by provider fiscal year and bill type

Usage notes
Required when the value of the Total DRG amount is not zero. If not required by this implementation guide, do not send.

This segment provides summary information specific to an iteration of the LX loop (Table 2).
Each element represents the total value for the provider/bill type combination in this loop 2000 iteration.
Required for Medicare Part A. If not required by this implementation guide, do not send.
Example
TS2*000000000*000000*000000*000000000000*000000*000000000000*0000000000*0000000000*000000000000000*000000000000000*0000000000*000000*00000000000000*0000000*000*000000000000000*0*000*00000000000~
TS2-01
782
Total DRG Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS201 is the total diagnosis related group (DRG) amount.
Usage notes
This includes: operating federal-specific amount, operating hospital-specific amount, operating Indirect Medical Education amount, and operating Disproportionate Share Hospital amount. It does not include any operating outlier amount.
See TR3 note 2.
TS2-02
782
Total Federal Specific Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS202 is the total federal specific amount.
Usage notes
See TR3 note 2.
TS2-03
782
Total Hospital Specific Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS203 is the total hospital specific amount.
Usage notes
See TR3 note 2.
TS2-04
782
Total Disproportionate Share Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS204 is the total disproportionate share amount.
Usage notes
See TR3 note 2.
TS2-05
782
Total Capital Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS205 is the total capital amount.
Usage notes
This includes: capital federal-specfic amount, hospital federal-specfic amount, hold harmless amount, Indirect Medical Education amount, Disproportionate Share Hospital amount, and the exception amount. It does not include any capital outlier amount.
See TR3 note 2.
TS2-06
782
Total Indirect Medical Education Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS206 is the total indirect medical education amount.
Usage notes
See TR3 note 2.
TS2-07
380
Total Outlier Day Count
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

TS207 is the total number of outlier days.
Usage notes
See TR3 note 2.
TS2-08
782
Total Day Outlier Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS208 is the total day outlier amount.
Usage notes
See TR3 note 2.
TS2-09
782
Total Cost Outlier Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS209 is the total cost outlier amount.
Usage notes
See TR3 note 2.
TS2-10
380
Average DRG Length of Stay
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

TS210 is the diagnosis related group (DRG) average length of stay.
Usage notes
See TR3 note 2.
TS2-11
380
Total Discharge Count
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

TS211 is the total number of discharges.
Usage notes
This is the discharge count produced by PPS PRICER SOFTWARE.
See TR3 note 2.
TS2-12
380
Total Cost Report Day Count
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

TS212 is the total number of cost report days.
Usage notes
See TR3 note 2.
TS2-13
380
Total Covered Day Count
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

TS213 is the total number of covered days.
Usage notes
See TR3 note 2.
TS2-14
380
Total Noncovered Day Count
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

TS214 is total number of non-covered days.
Usage notes
See TR3 note 2.
TS2-15
782
Total MSP Pass-Through Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS215 is the total Medicare Secondary Payer (MSP) pass- through amount calculated for a non-Medicare payer.
Usage notes
See TR3 note 2.
TS2-16
380
Average DRG weight
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

TS216 is the average diagnosis-related group (DRG) weight.
Usage notes
See TR3 note 2.
TS2-17
782
Total PPS Capital FSP DRG Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS217 is the total prospective payment system (PPS) capital, federal-specific portion, diagnosis-related group (DRG) amount.
Usage notes
See TR3 note 2.
TS2-18
782
Total PPS Capital HSP DRG Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS218 is the total prospective payment system (PPS) capital, hospital-specific portion, diagnosis-related group (DRG) amount.
Usage notes
See TR3 note 2.
TS2-19
782
Total PPS DSH DRG Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

TS219 is the total prospective payment system (PPS) disproportionate share, hospital diagnosis-related group (DRG) amount.
Usage notes
See TR3 note 2.
2100 Claim Payment Information Loop
Required
Max >1
CLP
0100
Claim Payment Information
Required
Max use 1
To supply information common to all services of a claim

Usage notes
For CLP segment occurrence limitations, see section 1.3.2, Other Usage Limitations.
Example
CLP*XXXX*20*0000000000000*0000*000000000000000*AM*XX*XX*X**XXX*0000000000*0000000~
CLP-01
1028
Patient Control Number
Required
String (AN)
Min 1
Max 38
Identifier used to track a claim from creation by the health care provider through payment

Usage notes
Use this number for the patient control number assigned by the provider. If the patient control number is not present on the incoming claim, enter a single zero. The value in CLP01 must be identical to any value received as a Claim Submitter's Identifier on the original claim (CLM01 of the ANSI ASC X12 837, if applicable). This data element is the primary key for posting the remittance information into the provider's database. In the case of pharmacy claims, this is the prescription reference number (field 402-02 in the NCPDP 5.1 format).
CLP-02
1029
Claim Status Code
Required
Identifier (ID)
Code identifying the status of an entire claim as assigned by the payor, claim review organization or repricing organization

Usage notes
To determine the full claim status reference Claim adjustment reason codes in the CAS segment in conjunction with this claim status code.
1
Processed as Primary
Use this code if the claim was adjudicated by the current payer as primary regardless of whether any part of the claim was paid.

2
Processed as Secondary
Use this code if the claim was adjudicated by the current payer as secondary regardless of whether any part of the claim was paid.

3
Processed as Tertiary
Use this code if the claim was adjudicated by the current payer as tertiary (or subsequent) regardless of whether any part of the claim was paid.

4
Denied
Usage of this code would apply if the Patient/Subscriber is not recognized, and the claim was not forwarded to another payer.

19
Processed as Primary, Forwarded to Additional Payer(s)
When this code is used, the Crossover Carrier Name NM1 segment is required.

20
Processed as Secondary, Forwarded to Additional Payer(s)
When this code is used, the Crossover Carrier Name NM1 segment is required.

21
Processed as Tertiary, Forwarded to Additional Payer(s)
When this code is used, the Crossover Carrier Name NM1 segment is required.

22
Reversal of Previous Payment
See section 1.10.2.8 for usage information.

23
Not Our Claim, Forwarded to Additional Payer(s)
Usage of this code would apply if the patient/subscriber is not recognized, the claim was not adjudicated by the payer, but other payers are known and the claim has been forwarded to another payer. When this code is used, the Crossover Carrier Name NM1 segment is required.

25
Predetermination Pricing Only - No Payment
CLP-03
782
Total Claim Charge Amount
Required
Decimal number (R)
Min 1
Max 15
Monetary amount

CLP03 is the amount of submitted charges this claim.
Usage notes
See 1.10.2.1, Balancing, in this implementation guide for additional information.
Use this monetary amount for the submitted charges for this claim. The amount can be positive, zero or negative. An example of a situation with a negative charge is a reversal claim. See section 1.10.2.8 for additional information.
Decimal elements will be limited to a maximum length of 10 characters including reported or implied places for cents (implied value of 00 after the decimal point). This applies to all subsequent 782 elements.
CLP-04
782
Claim Payment Amount
Required
Decimal number (R)
Min 1
Max 15
Monetary amount

CLP04 is the amount paid this claim.
Usage notes
See 1.10.2.1, Balancing, in this implementation guide for additional information. See section 1.10.2.9 for information about interest considerations.
Use this monetary amount for the amount paid for this claim. It can be positive, zero or negative, but the value in BPR02 may not be negative.
CLP-05
782
Patient Responsibility Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

CLP05 is the patient responsibility amount.
Usage notes
Amounts in CLP05 must have supporting adjustments reflected in CAS segments at the 2100 (CLP) or 2110 (SVC) loop level with a Claim Adjustment Group (CAS01) code of PR (Patient Responsibility).
Use this monetary amount for the payer's statement of the patient responsibility amount for this claim, which can include such items as deductible, non-covered services, co-pay and co-insurance. This is not used for reversals. See section 1.10.2.8, Reversals and Corrections, for additional information.
CLP-06
1032
Claim Filing Indicator Code
Required
Identifier (ID)
Code identifying type of claim

Usage notes
For many providers to electronically post the 835 remittance data to their patient accounting systems without human intervention, a unique, provider-specific insurance plan code is needed. This code allows the provider to separately identify and manage the different product lines or contractual arrangements between the payer and the provider. Because most payers maintain the same Originating Company Identifier in the TRN03 or BPR10 for all product lines or contractual relationships, the CLP06 is used by the provider as a table pointer in combination with the TRN03 or BPR10 to identify the unique, provider-specific insurance plan code needed to post the payment without human intervention. The value should mirror the value received in the original claim (2-005 SBR09 of the 837), if applicable, or provide the value as assigned or edited by the payer. For example the BL from the SBR09 in the 837 would be returned as 12, 13, 15, in the 835 when more details are known. The 837 SBR09 code CI (Commercial Insurance) is generic, if through adjudication the specific type of plan is obtained a more specific code must be returned in the 835.
The 837 and 835 transaction code lists for this element are not identical by design. There are some business differences between the two transactions. When a code from the 837 is not available in the 835 another valid code from the 835 must be assigned by the payer.
12
Preferred Provider Organization (PPO)
This code is also used for Blue Cross/Blue Shield participating provider arrangements.

13
Point of Service (POS)
14
Exclusive Provider Organization (EPO)
15
Indemnity Insurance
This code is also used for Blue Cross/Blue Shield non-participating provider arrangements.

16
Health Maintenance Organization (HMO) Medicare Risk
17
Dental Maintenance Organization
AM
Automobile Medical
CH
Champus
DS
Disability
HM
Health Maintenance Organization
LM
Liability Medical
MA
Medicare Part A
MB
Medicare Part B
MC
Medicaid
OF
Other Federal Program
Use this code for the Black Lung Program.

TV
Title V
VA
Veterans Affairs Plan
WC
Workers' Compensation Health Claim
ZZ
Mutually Defined
CLP-07
127
Payer Claim Control Number
Required
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

CLP07 is the payer's internal control number.
Usage notes
Use this number for the payer's internal control number. This number must apply to the entire claim.
CLP-08
1331
Facility Type Code
Optional
String (AN)
Min 1
Max 2
Code identifying where services were, or may be, performed; the first and second positions of the Uniform Bill Type Code for Institutional Services or the Place of Service Codes for Professional or Dental Services.

Usage notes
Since professional or dental claims can have different place of service codes for services within a single claim, default to the place of service of the first service line when the service lines are not all for the same place of service.
This number was received in CLM05-1 of the 837 claim.
CLP-09
1325
Claim Frequency Code
Optional
Identifier (ID)
Min 1
Max 1
Code specifying the frequency of the claim; this is the third position of the Uniform Billing Claim Form Bill Type

Usage notes
This number was received in CLM05-3 of the 837 Claim.
CLP-11
1354
Diagnosis Related Group (DRG) Code
Optional
Identifier (ID)
Min 1
Max 4
Code indicating a patient's diagnosis group based on a patient's illness, diseases, and medical problems

CLP-12
380
Diagnosis Related Group (DRG) Weight
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

CLP12 is the diagnosis-related group (DRG) weight.
Usage notes
This is the adjudicated DRG Weight.
CLP-13
954
Discharge Fraction
Optional
Decimal number (R)
Min 1
Max 10
Percentage expressed as a decimal (e.g., 0.0 through 1.0 represents 0% through 100%)

CLP13 is the discharge fraction.
Usage notes
This is the adjudicated discharge fraction.
CAS
0200
Claim Adjustment
Optional
Max use 99
To supply adjustment reason codes and amounts as needed for an entire claim or for a particular service within the claim being paid

Usage notes
Payers must use this CAS segment to report claim level adjustments that cause the amount paid to differ from the amount originally charged. See 1.10.2.1, Balancing, and 1.10.2.4, Claim Adjustment and Service Adjustment Segment Theory, for additional information.
See the SVC TR3 Note #1 for details about per diem adjustments.
A single CAS segment contains six repetitions of the "adjustment trio" composed of adjustment reason code, adjustment amount, and adjustment quantity. These six adjustment trios are used to report up to six adjustments related to a specific Claim Adjustment Group Code (CAS01). The six iterations (trios) of the Adjustment Reason Code related to the Specific Adjustment Group Code must be exhausted before repeating a second iteration of the CAS segment using the same Adjustment Group Code. The first adjustment must be the first non-zero adjustment and is reported in the first adjustment trio (CAS02-CAS04). If there is a second non-zero adjustment, it is reported in the second adjustment trio (CAS05-CAS07), and so on through the sixth adjustment trio (CAS17-CAS19).
Required when dollar amounts and/or quantities are being adjusted at the claim level. If not required by this implementation guide, do not send.
Example
CAS*PI*XXXXX*0*0000000*XXX*000000000*00000000*XXXXX*000000000*0000000*XXXXX*00000000*00000000000*XX*00000000000*00000000000*XX*00*000000000000000~
If Adjustment Reason Code (CAS-05) is present, then at least one of Adjustment Amount (CAS-06) or Adjustment Quantity (CAS-07) is required
If Adjustment Amount (CAS-06) is present, then Adjustment Reason Code (CAS-05) is required
If Adjustment Quantity (CAS-07) is present, then Adjustment Reason Code (CAS-05) is required
If Adjustment Reason Code (CAS-08) is present, then at least one of Adjustment Amount (CAS-09) or Adjustment Quantity (CAS-10) is required
If Adjustment Amount (CAS-09) is present, then Adjustment Reason Code (CAS-08) is required
If Adjustment Quantity (CAS-10) is present, then Adjustment Reason Code (CAS-08) is required
If Adjustment Reason Code (CAS-11) is present, then at least one of Adjustment Amount (CAS-12) or Adjustment Quantity (CAS-13) is required
If Adjustment Amount (CAS-12) is present, then Adjustment Reason Code (CAS-11) is required
If Adjustment Quantity (CAS-13) is present, then Adjustment Reason Code (CAS-11) is required
If Adjustment Reason Code (CAS-14) is present, then at least one of Adjustment Amount (CAS-15) or Adjustment Quantity (CAS-16) is required
If Adjustment Amount (CAS-15) is present, then Adjustment Reason Code (CAS-14) is required
If Adjustment Quantity (CAS-16) is present, then Adjustment Reason Code (CAS-14) is required
If Adjustment Reason Code (CAS-17) is present, then at least one of Adjustment Amount (CAS-18) or Adjustment Quantity (CAS-19) is required
If Adjustment Amount (CAS-18) is present, then Adjustment Reason Code (CAS-17) is required
If Adjustment Quantity (CAS-19) is present, then Adjustment Reason Code (CAS-17) is required
CAS-01
1033
Claim Adjustment Group Code
Required
Identifier (ID)
Code identifying the general category of payment adjustment

Usage notes
Evaluate the usage of group codes in CAS01 based on the following order for their applicability to a set of one or more adjustments: PR, CO, PI, OA. See 1.10.2.4, Claim Adjustment and Service Adjustment Segment Theory, for additional information. (Note: This does not mean that the adjustments must be reported in this order.)
CO
Contractual Obligations
Use this code when a joint payer/payee contractual agreement or a regulatory requirement resulted in an adjustment.

OA
Other adjustments
Avoid using the Other Adjustment Group Code (OA) except for business situations described in sections 1.10.2.6, 1.10.2.7 and 1.10.2.13.

PI
Payor Initiated Reductions
Use this code when, in the opinion of the payer, the adjustment is not the responsibility of the patient, but there is no supporting contract between the provider and the payer (i.e., medical review or professional review organization adjustments).

PR
Patient Responsibility
CAS-02
1034
Adjustment Reason Code
Required
Identifier (ID)
Min 1
Max 5
Code identifying the detailed reason the adjustment was made

Usage notes
Required to report a non-zero adjustment applied at the claim level for the claim adjustment group code reported in CAS01.
CAS-03
782
Adjustment Amount
Required
Decimal number (R)
Min 1
Max 15
Monetary amount

CAS03 is the amount of adjustment.
Usage notes
Use this monetary amount for the adjustment amount. A negative amount increases the payment, and a positive amount decreases the payment contained in CLP04.
Decimal elements will be limited to a maximum length of 10 characters including reported or implied places for cents (implied value of 00 after the decimal point). This applies to all subsequent 782 elements.
CAS-04
380
Adjustment Quantity
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

CAS04 is the units of service being adjusted.
Usage notes
See section 1.10.2.4.1 for additional information.
A positive value decreases the covered days, and a negative number increases the covered days.
CAS-05
1034
Adjustment Reason Code
Optional
Identifier (ID)
Min 1
Max 5
Code identifying the detailed reason the adjustment was made

CAS-06
782
Adjustment Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

CAS06 is the amount of the adjustment.
Usage notes
See CAS03.
CAS-07
380
Adjustment Quantity
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

CAS07 is the units of service being adjusted.
Usage notes
See CAS04.
CAS-08
1034
Adjustment Reason Code
Optional
Identifier (ID)
Min 1
Max 5
Code identifying the detailed reason the adjustment was made

CAS-09
782
Adjustment Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

CAS09 is the amount of the adjustment.
Usage notes
See CAS03.
CAS-10
380
Adjustment Quantity
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

CAS10 is the units of service being adjusted.
Usage notes
See CAS04.
CAS-11
1034
Adjustment Reason Code
Optional
Identifier (ID)
Min 1
Max 5
Code identifying the detailed reason the adjustment was made

CAS-12
782
Adjustment Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

CAS12 is the amount of the adjustment.
Usage notes
See CAS03.
CAS-13
380
Adjustment Quantity
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

CAS13 is the units of service being adjusted.
Usage notes
See CAS04.
CAS-14
1034
Adjustment Reason Code
Optional
Identifier (ID)
Min 1
Max 5
Code identifying the detailed reason the adjustment was made

CAS-15
782
Adjustment Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

CAS15 is the amount of the adjustment.
Usage notes
See CAS03.
CAS-16
380
Adjustment Quantity
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

CAS16 is the units of service being adjusted.
Usage notes
See CAS04.
CAS-17
1034
Adjustment Reason Code
Optional
Identifier (ID)
Min 1
Max 5
Code identifying the detailed reason the adjustment was made

CAS-18
782
Adjustment Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

CAS18 is the amount of the adjustment.
Usage notes
See CAS03.
CAS-19
380
Adjustment Quantity
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

CAS19 is the units of service being adjusted.
Usage notes
See CAS04.
NM1
0300
Patient Name
Required
Max use 1
To supply the full name of an individual or organizational entity

Usage notes
Provide the patient's identification number in NM109.
This segment must provide the information from the original claim. For example, when the claim is submitted as an ASC X12 837 transaction, this is the 2010CA loop NM1 Patient Name Segment unless not present on the original claim, then it is the 2010BA loop NM1 Subscriber name segment.
The Corrected Patient/Insured Name NM1 segment identifies the adjudicated Insured Name and ID information if different than what was submitted on the claim.
Example
NM1*QC*1*X*XXXX*X**XX*II*XX~
Variants (all may be used)
NM1Insured Name
NM1Corrected Patient/Insured Name
NM1Service Provider Name
NM1Crossover Carrier Name
NM1Corrected Priority Payer Name
NM1Other Subscriber Name
If either Identification Code Qualifier (NM1-08) or Patient Identifier (NM1-09) is present, then the other is required
NM1-01
98
Entity Identifier Code
Required
Identifier (ID)
Code identifying an organizational entity, a physical location, property or an individual

QC
Patient
NM1-02
1065
Entity Type Qualifier
Required
Identifier (ID)
Code qualifying the type of entity

NM102 qualifies NM103.
1
Person
NM1-03
1035
Patient Last Name
Optional
String (AN)
Min 1
Max 60
Individual last name or organizational name

NM1-04
1036
Patient First Name
Optional
String (AN)
Min 1
Max 35
Individual first name

NM1-05
1037
Patient Middle Name or Initial
Optional
String (AN)
Min 1
Max 25
Individual middle name or initial

Usage notes
If this data element is used and contains only one character, it is assumed to represent the middle initial.
NM1-07
1039
Patient Name Suffix
Optional
String (AN)
Min 1
Max 10
Suffix to individual name

Usage notes
An example of this is when a Junior and Senior are covered under the same subscriber.
NM1-08
66
Identification Code Qualifier
Optional
Identifier (ID)
Code designating the system/method of code structure used for Identification Code (67)

34
Social Security Number
HN
Health Insurance Claim (HIC) Number
II
Standard Unique Health Identifier for each Individual in the United States
Use this code if mandated in a final Federal Rule.

MI
Member Identification Number
MR
Medicaid Recipient Identification Number
NM1-09
67
Patient Identifier
Optional
String (AN)
Min 2
Max 80
Code identifying a party or other code

NM1
0300
Insured Name
Optional
Max use 1
To supply the full name of an individual or organizational entity

Usage notes
In the case of Medicare and Medicaid, the insured patient is always the subscriber and this segment is not used.
Required when the original claim reported the insured or subscriber (for example 837 2010BA loop Subscriber Name NM1 Segment) that is different from the patient. If not required by this implementation guide, do not send.
This segment contains the same information as reported on the claim (for example 837 2010BA loop Subscriber Name NM1 Segment when the patient was reported in the 2010CA loop Patient Name NM1 Segment).
Example
NM1*IL*2*XXXXXX*XXXX*X**XXXXX*II*XXXX~
Variants (all may be used)
NM1Patient Name
NM1Corrected Patient/Insured Name
NM1Service Provider Name
NM1Crossover Carrier Name
NM1Corrected Priority Payer Name
NM1Other Subscriber Name
NM1-01
98
Entity Identifier Code
Required
Identifier (ID)
Code identifying an organizational entity, a physical location, property or an individual

IL
Insured or Subscriber
NM1-02
1065
Entity Type Qualifier
Required
Identifier (ID)
Code qualifying the type of entity

NM102 qualifies NM103.
1
Person
2
Non-Person Entity
NM1-03
1035
Subscriber Last Name
Optional
String (AN)
Min 1
Max 60
Individual last name or organizational name

NM1-04
1036
Subscriber First Name
Optional
String (AN)
Min 1
Max 35
Individual first name

NM1-05
1037
Subscriber Middle Name or Initial
Optional
String (AN)
Min 1
Max 25
Individual middle name or initial

Usage notes
If this data element is used and contains only one character, it is assumed to represent the middle initial.
NM1-07
1039
Subscriber Name Suffix
Optional
String (AN)
Min 1
Max 10
Suffix to individual name

Usage notes
For example, use when necessary to differentiate between a Junior and Senior under the same contract.
NM1-08
66
Identification Code Qualifier
Required
Identifier (ID)
Code designating the system/method of code structure used for Identification Code (67)

FI
Federal Taxpayer's Identification Number
Not Used when NM102=1.

II
Standard Unique Health Identifier for each Individual in the United States
Use this code if mandated in a final Federal Rule.

MI
Member Identification Number
The code MI is intended to identify that the subscriber's identification number as assigned by the payer will be conveyed in NM109. Payers use different terminology to convey the same number, therefore, the 835 workgroup recommends using MI (Member Identification number) to convey the same categories of numbers as represented in the 837 IGs for the inbound claims.

NM1-09
67
Subscriber Identifier
Required
String (AN)
Min 2
Max 80
Code identifying a party or other code

NM1
0300
Corrected Patient/Insured Name
Optional
Max use 1
To supply the full name of an individual or organizational entity

Usage notes
Since the patient is always the insured for Medicare and Medicaid, this segment always provides corrected patient information for Medicare and Medicaid. For other carriers, this will always be the corrected insured information.
Required when needed to provide corrected information about the patient or insured. If not required by this implementation guide, do not send.
Example
NM1*74*1*XX*XXX*XXXXX**XXXXX*C*XXXXXXX~
Variants (all may be used)
NM1Patient Name
NM1Insured Name
NM1Service Provider Name
NM1Crossover Carrier Name
NM1Corrected Priority Payer Name
NM1Other Subscriber Name
If either Identification Code Qualifier (NM1-08) or Corrected Insured Identification Indicator (NM1-09) is present, then the other is required
NM1-01
98
Entity Identifier Code
Required
Identifier (ID)
Code identifying an organizational entity, a physical location, property or an individual

74
Corrected Insured
NM1-02
1065
Entity Type Qualifier
Required
Identifier (ID)
Code qualifying the type of entity

NM102 qualifies NM103.
1
Person
2
Non-Person Entity
NM1-03
1035
Corrected Patient or Insured Last Name
Optional
String (AN)
Min 1
Max 60
Individual last name or organizational name

NM1-04
1036
Corrected Patient or Insured First Name
Optional
String (AN)
Min 1
Max 35
Individual first name

NM1-05
1037
Corrected Patient or Insured Middle Name
Optional
String (AN)
Min 1
Max 25
Individual middle name or initial

Usage notes
If this data element is used and contains only one character, it is assumed to represent the middle initial.
NM1-07
1039
Corrected Patient or Insured Name Suffix
Optional
String (AN)
Min 1
Max 10
Suffix to individual name

NM1-08
66
Identification Code Qualifier
Optional
Identifier (ID)
Code designating the system/method of code structure used for Identification Code (67)

C
Insured's Changed Unique Identification Number
NM1-09
67
Corrected Insured Identification Indicator
Optional
String (AN)
Min 2
Max 80
Code identifying a party or other code

NM1
0300
Service Provider Name
Optional
Max use 1
To supply the full name of an individual or organizational entity

Usage notes
This segment provides information about the rendering provider. An identification number is provided in NM109.
This information is provided to facilitate identification of the claim within a payee's system. Other providers (e.g., Referring provider, supervising provider) related to the claim but not directly related to the payment are not supported and are not necessary for claim identification.
Required when the rendering provider is different from the payee. If not required by this implementation guide, do not send.
Example
NM1*82*2*XXXX*XXXXXX*X**XXXX*FI*XXXXXXX~
Variants (all may be used)
NM1Patient Name
NM1Insured Name
NM1Corrected Patient/Insured Name
NM1Crossover Carrier Name
NM1Corrected Priority Payer Name
NM1Other Subscriber Name
NM1-01
98
Entity Identifier Code
Required
Identifier (ID)
Code identifying an organizational entity, a physical location, property or an individual

82
Rendering Provider
NM1-02
1065
Entity Type Qualifier
Required
Identifier (ID)
Code qualifying the type of entity

NM102 qualifies NM103.
1
Person
2
Non-Person Entity
NM1-03
1035
Rendering Provider Last or Organization Name
Optional
String (AN)
Min 1
Max 60
Individual last name or organizational name

NM1-04
1036
Rendering Provider First Name
Optional
String (AN)
Min 1
Max 35
Individual first name

NM1-05
1037
Rendering Provider Middle Name or Initial
Optional
String (AN)
Min 1
Max 25
Individual middle name or initial

Usage notes
If this data element is used and contains only one character, it represents the middle initial.
NM1-07
1039
Rendering Provider Name Suffix
Optional
String (AN)
Min 1
Max 10
Suffix to individual name

NM1-08
66
Identification Code Qualifier
Required
Identifier (ID)
Code designating the system/method of code structure used for Identification Code (67)

BD
Blue Cross Provider Number
BS
Blue Shield Provider Number
FI
Federal Taxpayer's Identification Number
This is the preferred ID until the National Provider ID is mandated and applicable.
For individual providers as payees, use this qualifier to represent the Social Security Number.

MC
Medicaid Provider Number
PC
Provider Commercial Number
SL
State License Number
UP
Unique Physician Identification Number (UPIN)
XX
Centers for Medicare and Medicaid Services National Provider Identifier
Required value if the National Provider ID is mandated for use and the provider is a covered health care provider under the mandate. Otherwise, one of the other listed codes may be used.

NM1-09
67
Rendering Provider Identifier
Required
String (AN)
Min 2
Max 80
Code identifying a party or other code

NM1
0300
Crossover Carrier Name
Optional
Max use 1
To supply the full name of an individual or organizational entity

Usage notes
This segment provides information about the crossover carrier. Provide any reference numbers in NM109. The crossover carrier is defined as any payer to which the claim is transferred for further payment after being finalized by the current payer.
Required when the claim is transferred to another carrier or coverage (CLP02 equals 19, 20, 21 or 23). If not required by this implementation guide, do not send.
Example
NM1*TT*2*XXXXX*****AD*XXXXXXX~
Variants (all may be used)
NM1Patient Name
NM1Insured Name
NM1Corrected Patient/Insured Name
NM1Service Provider Name
NM1Corrected Priority Payer Name
NM1Other Subscriber Name
NM1-01
98
Entity Identifier Code
Required
Identifier (ID)
Code identifying an organizational entity, a physical location, property or an individual

TT
Transfer To
NM1-02
1065
Entity Type Qualifier
Required
Identifier (ID)
Code qualifying the type of entity

NM102 qualifies NM103.
2
Non-Person Entity
NM1-03
1035
Crossover Carrier Name
Required
String (AN)
Min 1
Max 60
Individual last name or organizational name

Usage notes
Name of the crossover carrier associated with this claim.
NM1-08
66
Identification Code Qualifier
Required
Identifier (ID)
Code designating the system/method of code structure used for Identification Code (67)

AD
Blue Cross Blue Shield Association Plan Code
FI
Federal Taxpayer's Identification Number
NI
National Association of Insurance Commissioners (NAIC) Identification
This is the preferred ID unless XV is used.

PI
Payor Identification
PP
Pharmacy Processor Number
XV
Centers for Medicare and Medicaid Services PlanID
Use when reporting Health Plan ID (HPID) or Other Entity Identifier (OEID). Otherwise, one of the other listed codes may be used.

NM1-09
67
Crossover Carrier Identifier
Required
String (AN)
Min 2
Max 80
Code identifying a party or other code

NM1
0300
Corrected Priority Payer Name
Optional
Max use 1
To supply the full name of an individual or organizational entity

Usage notes
Provide any reference numbers in NM109. Use of this segment identifies the priority payer. Do not use this segment when the Crossover Carrier NM1 segment is used.
Required when current payer believes that another payer has priority for making a payment and the claim is not being automatically transferred to that payer. If not required by this implementation guide, do not send.
Example
NM1*PR*2*XXXXX*****PP*XXX~
Variants (all may be used)
NM1Patient Name
NM1Insured Name
NM1Corrected Patient/Insured Name
NM1Service Provider Name
NM1Crossover Carrier Name
NM1Other Subscriber Name
NM1-01
98
Entity Identifier Code
Required
Identifier (ID)
Code identifying an organizational entity, a physical location, property or an individual

PR
Payer
NM1-02
1065
Entity Type Qualifier
Required
Identifier (ID)
Code qualifying the type of entity

NM102 qualifies NM103.
2
Non-Person Entity
NM1-03
1035
Corrected Priority Payer Name
Required
String (AN)
Min 1
Max 60
Individual last name or organizational name

NM1-08
66
Identification Code Qualifier
Required
Identifier (ID)
Code designating the system/method of code structure used for Identification Code (67)

AD
Blue Cross Blue Shield Association Plan Code
FI
Federal Taxpayer's Identification Number
NI
National Association of Insurance Commissioners (NAIC) Identification
This is the preferred ID unless XV is used.

PI
Payor Identification
PP
Pharmacy Processor Number
XV
Centers for Medicare and Medicaid Services PlanID
Use when reporting Health Plan ID (HPID) or Other Entity Identifier (OEID). Otherwise, one of the other listed codes may be used.

NM1-09
67
Corrected Priority Payer Identification Number
Required
String (AN)
Min 2
Max 80
Code identifying a party or other code

NM1
0300
Other Subscriber Name
Optional
Max use 1
To supply the full name of an individual or organizational entity

Usage notes
This is the name and ID number of the other subscriber when a corrected priority payer has been identified. When used, either the name or ID must be supplied.
Required when a corrected priority payer has been identified in another NM1 segment AND the name or ID of the other subscriber is known. If not required by this implementation guide, do not send.
Example
NM1*GB*2*X*XX*XXXXXX**XXX*MI*XXXXXX~
Variants (all may be used)
NM1Patient Name
NM1Insured Name
NM1Corrected Patient/Insured Name
NM1Service Provider Name
NM1Crossover Carrier Name
NM1Corrected Priority Payer Name
If either Identification Code Qualifier (NM1-08) or Other Subscriber Identifier (NM1-09) is present, then the other is required
NM1-01
98
Entity Identifier Code
Required
Identifier (ID)
Code identifying an organizational entity, a physical location, property or an individual

GB
Other Insured
NM1-02
1065
Entity Type Qualifier
Required
Identifier (ID)
Code qualifying the type of entity

NM102 qualifies NM103.
1
Person
2
Non-Person Entity
NM1-03
1035
Other Subscriber Last Name
Optional
String (AN)
Min 1
Max 60
Individual last name or organizational name

Usage notes
At least one of NM103 or NM109 must be present.
NM1-04
1036
Other Subscriber First Name
Optional
String (AN)
Min 1
Max 35
Individual first name

NM1-05
1037
Other Subscriber Middle Name or Initial
Optional
String (AN)
Min 1
Max 25
Individual middle name or initial

Usage notes
When only one character is present this is assumed to be the middle initial.
NM1-07
1039
Other Subscriber Name Suffix
Optional
String (AN)
Min 1
Max 10
Suffix to individual name

NM1-08
66
Identification Code Qualifier
Optional
Identifier (ID)
Code designating the system/method of code structure used for Identification Code (67)

FI
Federal Taxpayer's Identification Number
Not Used when NM102=1.

II
Standard Unique Health Identifier for each Individual in the United States
Use this code if mandated in a final Federal Rule.

MI
Member Identification Number
Use this code when supplying the number used for identification of the subscriber in NM109.

NM1-09
67
Other Subscriber Identifier
Optional
String (AN)
Min 2
Max 80
Code identifying a party or other code

Usage notes
At least one of NM103 or NM109 must be present.
MIA
0330
Inpatient Adjudication Information
Optional
Max use 1
To provide claim-level data related to the adjudication of Medicare inpatient claims

Usage notes
When used outside of the Medicare and Medicaid community only MIA01, 05, 20, 21, 22 and 23 may be used.
Either MIA or MOA may appear, but not both.
This segment must not be used for covered days or lifetime reserve days. For covered or lifetime reserve days, use the Supplemental Claim Information Quantities Segment in the Claim Payment Loop.
All situational quantities and/or monetary amounts in this segment are required when the value of the item is different than zero.
Required for all inpatient claims when there is a need to report Remittance Advice Remark Codes at the claim level or, the claim is paid by Medicare or Medicaid under the Prospective Payment System (PPS). If not required by this implementation guide, do not send.
Example
MIA*000000000000000*00000000*000000000*000000000000000*XXX*000*000000000000*000000*000000*0*000000000000000*00000000*0000000000000*00000000000000*000*00000000*0000000000000*00000*00000000*XXXX*XXXXX*XX*XXXX*000000000~
MIA-01
380
Covered Days or Visits Count
Required
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

MIA01 is the covered days.
Usage notes
Implementers utilizing the MIA segment always transmit the number zero. See the QTY segment at the claim level for covered days or visits count.
MIA-02
782
PPS Operating Outlier Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MIA02 is the Prospective Payment System (PPS) Operating Outlier amount.
Usage notes
See TR3 note 4.
Decimal elements will be limited to a maximum length of 10 characters including reported or implied places for cents (implied value of 00 after the decimal point). This applies to all subsequent 782 elements.
MIA-03
380
Lifetime Psychiatric Days Count
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

MIA03 is the lifetime psychiatric days.
MIA-04
782
Claim DRG Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MIA04 is the Diagnosis Related Group (DRG) amount.
MIA-05
127
Claim Payment Remark Code
Optional
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

MIA05 is the Claim Payment Remark Code. See Code Source 411.
MIA-06
782
Claim Disproportionate Share Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MIA06 is the disproportionate share amount.
MIA-07
782
Claim MSP Pass-through Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MIA07 is the Medicare Secondary Payer (MSP) pass-through amount.
MIA-08
782
Claim PPS Capital Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MIA08 is the total Prospective Payment System (PPS) capital amount.
MIA-09
782
PPS-Capital FSP DRG Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MIA09 is the Prospective Payment System (PPS) capital, federal specific portion, Diagnosis Related Group (DRG) amount.
MIA-10
782
PPS-Capital HSP DRG Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MIA10 is the Prospective Payment System (PPS) capital, hospital specific portion, Diagnosis Related Group (DRG), amount.
MIA-11
782
PPS-Capital DSH DRG Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MIA11 is the Prospective Payment System (PPS) capital, disproportionate share, hospital Diagnosis Related Group (DRG) amount.
MIA-12
782
Old Capital Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MIA12 is the old capital amount.
MIA-13
782
PPS-Capital IME amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MIA13 is the Prospective Payment System (PPS) capital indirect medical education claim amount.
MIA-14
782
PPS-Operating Hospital Specific DRG Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MIA14 is hospital specific Diagnosis Related Group (DRG) Amount.
MIA-15
380
Cost Report Day Count
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

MIA15 is the cost report days.
MIA-16
782
PPS-Operating Federal Specific DRG Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MIA16 is the federal specific Diagnosis Related Group (DRG) amount.
MIA-17
782
Claim PPS Capital Outlier Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MIA17 is the Prospective Payment System (PPS) Capital Outlier amount.
MIA-18
782
Claim Indirect Teaching Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MIA18 is the indirect teaching amount.
MIA-19
782
Nonpayable Professional Component Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MIA19 is the professional component amount billed but not payable.
MIA-20
127
Claim Payment Remark Code
Optional
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

MIA20 is the Claim Payment Remark Code. See Code Source 411.
MIA-21
127
Claim Payment Remark Code
Optional
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

MIA21 is the Claim Payment Remark Code. See Code Source 411.
MIA-22
127
Claim Payment Remark Code
Optional
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

MIA22 is the Claim Payment Remark Code. See Code Source 411.
MIA-23
127
Claim Payment Remark Code
Optional
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

MIA23 is the Claim Payment Remark Code. See Code Source 411.
MIA-24
782
PPS-Capital Exception Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MIA24 is the capital exception amount.
MOA
0350
Outpatient Adjudication Information
Optional
Max use 1
To convey claim-level data related to the adjudication of Medicare claims not related to an inpatient setting

Usage notes
Required when the outpatient institutional claim reimbursement rate is not zero for a Medicare or Medicaid claim. If not required by this implementation guide, do not send.

Required for outpatient/professional claims where there is a need to report a Remittance Advice Remark Code at the claim level or when the payer is Medicare or Medicaid and MOA01, 02, 08 or 09 are non-zero. If not required by this implementation guide, do not send.
Either MIA or MOA may appear, but not both.
All situational quantities and/or monetary amounts in this segment are;required when the value of the item is different than zero.
Example
MOA*0000000000*00000000*XXXX*XX*XXX*XXXXX*XXX*0000000000*000~
MOA-01
954
Reimbursement Rate
Optional
Decimal number (R)
Min 1
Max 10
Percentage expressed as a decimal (e.g., 0.0 through 1.0 represents 0% through 100%)

MOA01 is the reimbursement rate.
MOA-02
782
Claim HCPCS Payable Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MOA02 is the claim Health Care Financing Administration Common Procedural Coding System (HCPCS) payable amount.
Usage notes
Decimal elements will be limited to a maximum length of 10 characters including reported or implied places for cents (implied value of 00 after the decimal point). This applies to all subsequent 782 elements.
MOA-03
127
Claim Payment Remark Code
Optional
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

MOA03 is the Claim Payment Remark Code. See Code Source 411.
MOA-04
127
Claim Payment Remark Code
Optional
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

MOA04 is the Claim Payment Remark Code. See Code Source 411.
MOA-05
127
Claim Payment Remark Code
Optional
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

MOA05 is the Claim Payment Remark Code. See Code Source 411.
MOA-06
127
Claim Payment Remark Code
Optional
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

MOA06 is the Claim Payment Remark Code. See Code Source 411.
MOA-07
127
Claim Payment Remark Code
Optional
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

MOA07 is the Claim Payment Remark Code. See Code Source 411.
MOA-08
782
Claim ESRD Payment Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MOA08 is the End Stage Renal Disease (ESRD) payment amount.
MOA-09
782
Nonpayable Professional Component Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

MOA09 is the professional component amount billed but not payable.
REF
0400
Other Claim Related Identification
Optional
Max use 5
To specify identifying information

Usage notes
Required when additional reference numbers specific to the claim in the CLP segment are provided to identify information used in the process of adjudicating this claim. If not required by this implementation guide, do not send.
Example
REF*9A*XXXX~
Variants (all may be used)
REFRendering Provider Identification
REF-01
128
Reference Identification Qualifier
Required
Identifier (ID)
Code qualifying the Reference Identification

1L
Group or Policy Number
Use this code when conveying the Group Number in REF02.

1W
Member Identification Number
6P
Group Number
This is the Other Insured Group Number. This is required when a Corrected Priority Payer is identified in the NM1 segment and the Group Number of the other insured for that payer is known.

9A
Repriced Claim Reference Number
9C
Adjusted Repriced Claim Reference Number
28
Employee Identification Number
BB
Authorization Number
Use this qualifier only when supplying an authorization number that was assigned by the adjudication process and was not provided prior to the services. Do not use this qualifier when reporting the same number as reported in the claim as the prior authorization or pre-authorization number.

CE
Class of Contract Code
See section 1.10.2.15 for information on the use of Class of Contract Code.

EA
Medical Record Identification Number
F8
Original Reference Number
When this is a correction claim and CLP07 does not equal the CLP07 value from the original claim payment, one iteration of this REF segment using this qualifier is REQUIRED to identify the original claim CLP07 value in REF02. See section 1.10.2.8, Reversals and Corrections, for additional information.

G1
Prior Authorization Number
Use this qualifier when reporting the number received with the original claim as a pre-authorization number (in the 837 that was at table 2, position 180, REF segment, using the same qualifier of G1).

G3
Predetermination of Benefits Identification Number
IG
Insurance Policy Number
Use this code when conveying the Policy Number in REF02.

SY
Social Security Number
REF-02
127
Other Claim Related Identifier
Required
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

REF
0400
Rendering Provider Identification
Optional
Max use 10
To specify identifying information

Usage notes
The NM1 segment always contains the primary reference number.
Required when additional rendering provider identification numbers not already reported in the Provider NM1 segment for this claim were submitted on the original claim and impacted adjudication. If not required by this implementation guide, do not send.
Example
REF*1D*XX~
Variants (all may be used)
REFOther Claim Related Identification
REF-01
128
Reference Identification Qualifier
Required
Identifier (ID)
Code qualifying the Reference Identification

0B
State License Number
1A
Blue Cross Provider Number
1B
Blue Shield Provider Number
1C
Medicare Provider Number
1D
Medicaid Provider Number
1G
Provider UPIN Number
1H
CHAMPUS Identification Number
1J
Facility ID Number
D3
National Council for Prescription Drug Programs Pharmacy Number
G2
Provider Commercial Number
LU
Location Number
REF-02
127
Rendering Provider Secondary Identifier
Required
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

DTM
0500
Statement From or To Date
Optional
Max use 2
To specify pertinent dates and times

Usage notes
Dates at the claim level apply to the entire claim, including all service lines. Dates at the service line level apply only to the service line where they appear.
When claim dates are not provided, service dates are required for every service line.
When claim dates are provided, service dates are not required, but if used they override the claim dates for individual service lines.
For retail pharmacy claims, the Claim Statement Period Start Date is equivalent to the prescription filled date.
Required when the "Statement From or To Dates" are not supplied at the service (2110 loop) level. If not required by this implementation guide, may be provided at senders discretion, but cannot be required by the receiver.
For predeterminations, where there is no service date, the value of DTM02 must be 19000101. Use only when the CLP02 value is 25 - Predetermination Pricing Only - No Payment.
When payment is being made in advance of services, the use of future dates is allowed.
Example
DTM*233*20250519~
Variants (all may be used)
DTMCoverage Expiration Date
DTMClaim Received Date
DTM-01
374
Date Time Qualifier
Required
Identifier (ID)
Code specifying type of date or time, or both date and time

232
Claim Statement Period Start
If the claim statement period start date is conveyed without a subsequent claim statement period end date, the end date is assumed to be the same as the start date. This date or code 233 is required when service level dates are not provided in the remittance advice.

233
Claim Statement Period End
If a claim statement period end date is conveyed without a claim statement period start date, then the start date is assumed to be different from the end date but not conveyed at the payer's discretion. See the note on code 232.

DTM-02
373
Claim Date
Required
Date (DT)
CCYYMMDD format
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year

DTM
0500
Coverage Expiration Date
Optional
Max use 1
To specify pertinent dates and times

Usage notes
Required when payment is denied because of the expiration of coverage. If not required by this implementation guide, do not send.
Example
DTM*036*20250519~
Variants (all may be used)
DTMStatement From or To Date
DTMClaim Received Date
DTM-01
374
Date Time Qualifier
Required
Identifier (ID)
Code specifying type of date or time, or both date and time

036
Expiration
DTM-02
373
Date
Required
Date (DT)
CCYYMMDD format
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year

Usage notes
This is the expiration date of the patient's coverage.
DTM
0500
Claim Received Date
Optional
Max use 1
To specify pertinent dates and times

Usage notes
Required whenever state or federal regulations or the provider contract mandate interest payment or prompt payment discounts based upon the receipt date of the claim by the payer. If not required by this implementation guide, may be provided at sender's discretion, but cannot be required by the receiver.
Example
DTM*050*20250519~
Variants (all may be used)
DTMStatement From or To Date
DTMCoverage Expiration Date
DTM-01
374
Date Time Qualifier
Required
Identifier (ID)
Code specifying type of date or time, or both date and time

050
Received
DTM-02
373
Date
Required
Date (DT)
CCYYMMDD format
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year

Usage notes
This is the date that the claim was received by the payer.
PER
0600
Claim Contact Information
Optional
Max use 2
To identify a person or office to whom administrative communications should be directed

Usage notes
Required when there is a claim specific communications contact. If not required by this implementation guide, do not send.
When the communication number represents a telephone number in the United States and other countries using the North American Dialing Plan (for voice, data, fax, etc.), the communication number always includes the area code and phone number using the format AAABBBCCCC. Where AAA is the area code, BBB is the telephone number prefix, and CCCC is the telephone number (e.g. (800)555-1212 would be represented as 8005551212). The extension number, when applicable, is identified in the next element pair (Communications Number Qualifier and Communication Number) immediately after the telephone number.
Example
PER*CX*XX*FX*X*EX*XXXXX*EX*XXXX~
If either Communication Number Qualifier (PER-05) or Claim Contact Communications Number (PER-06) is present, then the other is required
If either Communication Number Qualifier (PER-07) or Communication Number Extension (PER-08) is present, then the other is required
PER-01
366
Contact Function Code
Required
Identifier (ID)
Code identifying the major duty or responsibility of the person or group named

CX
Payers Claim Office
PER-02
93
Claim Contact Name
Optional
String (AN)
Min 1
Max 60
Free-form name

PER-03
365
Communication Number Qualifier
Required
Identifier (ID)
Code identifying the type of communication number

EM
Electronic Mail
FX
Facsimile
TE
Telephone
PER-04
364
Claim Contact Communications Number
Required
String (AN)
Min 1
Max 256
Complete communications number including country or area code when applicable

PER-05
365
Communication Number Qualifier
Optional
Identifier (ID)
Code identifying the type of communication number

EM
Electronic Mail
EX
Telephone Extension
When used, the value following this code is the extension for the preceding communications contact number.

FX
Facsimile
TE
Telephone
PER-06
364
Claim Contact Communications Number
Optional
String (AN)
Min 1
Max 256
Complete communications number including country or area code when applicable

PER-07
365
Communication Number Qualifier
Optional
Identifier (ID)
Code identifying the type of communication number

EX
Telephone Extension
PER-08
364
Communication Number Extension
Optional
String (AN)
Min 1
Max 256
Complete communications number including country or area code when applicable

AMT
0620
Claim Supplemental Information
Optional
Max use 13
To indicate the total monetary amount

Usage notes
Use this segment to convey information only. It is not part of the financial balancing of the 835.
Send/receive one AMT for each applicable non-zero value. Do not report any zero values.
Required when the value of any specific amount identified by the AMT01 qualifier is non-zero. If not required by this implementation guide, do not send.
Example
AMT*ZM*000~
AMT-01
522
Amount Qualifier Code
Required
Identifier (ID)
Code to qualify amount

AU
Coverage Amount
Use this monetary amount to report the total covered charges.

This is the sum of the original submitted provider charges that are considered for payment under the benefit provisions of the health plan. This excludes charges considered not covered (i.e. per day television or telephone charges) but includes reductions to payments of covered services (i.e. reductions for amounts over fee schedule and patient deductibles).

D8
Discount Amount
Prompt Pay Discount Amount

See section 1.10.2.9 for additional information.

DY
Per Day Limit
F5
Patient Amount Paid
Use this monetary amount for the amount the patient has already paid.

I
Interest
See section 1.10.2.9 for additional information.

NL
Negative Ledger Balance
Used only by Medicare Part A and Medicare Part B.

T
Tax
T2
Total Claim Before Taxes
Used only when tax also applies to the claim.

ZK
Federal Medicare or Medicaid Payment Mandate - Category 1
ZL
Federal Medicare or Medicaid Payment Mandate - Category 2
ZM
Federal Medicare or Medicaid Payment Mandate - Category 3
ZN
Federal Medicare or Medicaid Payment Mandate - Category 4
ZO
Federal Medicare or Medicaid Payment Mandate - Category 5
AMT-02
782
Claim Supplemental Information Amount
Required
Decimal number (R)
Min 1
Max 15
Monetary amount

Usage notes
Decimal elements will be limited to a maximum length of 10 characters including reported or implied places for cents (implied value of 00 after the decimal point). This applies to all subsequent 782 elements.
QTY
0640
Claim Supplemental Information Quantity
Optional
Max use 14
To specify quantity information

Usage notes
Use this segment to convey information only. It is not part of the financial balancing of the 835.
Send one QTY for each non-zero value. Do not report any zero values.
Required when the value of a specific quantity identified by the QTY01 qualifier is non-zero. If not required by this implementation guide, do not send.
Example
QTY*PS*000000000~
QTY-01
673
Quantity Qualifier
Required
Identifier (ID)
Code specifying the type of quantity

CA
Covered - Actual
CD
Co-insured - Actual
LA
Life-time Reserve - Actual
LE
Life-time Reserve - Estimated
NE
Non-Covered - Estimated
NR
Not Replaced Blood Units
OU
Outlier Days
PS
Prescription
VS
Visits
ZK
Federal Medicare or Medicaid Payment Mandate - Category 1
ZL
Federal Medicare or Medicaid Payment Mandate - Category 2
ZM
Federal Medicare or Medicaid Payment Mandate - Category 3
ZN
Federal Medicare or Medicaid Payment Mandate - Category 4
ZO
Federal Medicare or Medicaid Payment Mandate - Category 5
QTY-02
380
Claim Supplemental Information Quantity
Required
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

2110 Service Payment Information Loop
Optional
Max 999
SVC
0700
Service Payment Information
Required
Max use 1
To supply payment and control information to a provider for a particular service

Usage notes
See section 1.10.2.1.1 (Service Line Balancing) for additional information.
The exception to the situational rule occurs with institutional claims when the room per diem is the only service line adjustment. In this instance, a claim level CAS adjustment to the per diem is appropriate (i.e., CASCO78*25~). See section 1.10.2.4.1 for additional information.
See 1.10.2.6, Procedure Code Bundling and Unbundling, and section 1.10.2.1.1, Service Line Balancing, for important SVC segment usage information.
Required for all service lines in a professional, dental or outpatient claim priced at the service line level or whenever payment for any service line of the claim is different than the original submitted charges due to service line specific adjustments (excluding cases where the only service specific adjustment is for room per diem). If not required by this implementation guide, do not send.
Example
SVC*N6>XXXXX>XX>XX>XX>XX*0000*000000*XXXX*00000*NU>XXXXX>XX>XX>XX>XX>X*000~
SVC-01
C003
Composite Medical Procedure Identifier
Required
Max use 1
To identify a medical procedure by its standardized codes and applicable modifiers

- SVC01 is the medical procedure upon which adjudication is based.
- For Medicare Part A claims, SVC01 would be the Health Care Financing Administration (HCFA) Common Procedural Coding System (HCPCS) Code (see code source 130) and SVC04 would be the Revenue Code (see code source 132).
C003-01
235
Product or Service ID Qualifier
Required
Identifier (ID)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)

C003-01 qualifies C003-02 and C003-08.
Usage notes
The value in SVC01-1 qualifies the values in SVC01-2, SVC01-3, SVC01-4, SVC01-5, SVC01-6 and SVC01-7.
AD
American Dental Association Codes
ER
Jurisdiction Specific Procedure and Supply Codes
HC
Health Care Financing Administration Common Procedural Coding System (HCPCS) Codes
Because the CPT codes of the American Medical Association are also level 1 HCPCS codes, they are reported under the code HC.

HP
Health Insurance Prospective Payment System (HIPPS) Skilled Nursing Facility Rate Code
Medicare uses this code to reflect the Skilled Nursing Facility Group as well as the Home Health Agency Outpatient Prospective Payment System.

IV
Home Infusion EDI Coalition (HIEC) Product/Service Code
This code set is not allowed for use under HIPAA at the time of this writing. The qualifier can only be used 1) If a new rule names HIEC as an allowable code set under HIPAA. 2) For Property & Casualty claims/encounters that are not covered under HIPAA.

N4
National Drug Code in 5-4-2 Format
N6
National Health Related Item Code in 4-6 Format
This code set is not allowed for use under HIPAA at the time of this writing. The qualifier can only be used 1) If a new rule names National Health Related Item Code in 4-6 Format Codes as an allowable code set under HIPAA. 2) For Property & Casualty claims/encounters that are not covered under HIPAA.

NU
National Uniform Billing Committee (NUBC) UB92 Codes
UI
U.P.C. Consumer Package Code (1-5-5)
This code set is not allowed for use under HIPAA at the time of this writing. The qualifier can only be used 1) If a new rule names U.P.C. Consumer Package Code (1-5-5) Codes as an allowable code set under HIPAA. 2) For Property & Casualty claims/encounters that are not covered under HIPAA.

WK
Advanced Billing Concepts (ABC) Codes
This code set is not allowed for use under HIPAA at the time of this writing. The qualifier can only be used in transactions covered under HIPAA by parties registered in the pilot project and their trading partners.

C003-02
234
Adjudicated Procedure Code
Required
String (AN)
Min 1
Max 48
Identifying number for a product or service

If C003-08 is used, then C003-02 represents the beginning value in the range in which the code occurs.
Usage notes
This is the adjudicated procedure code or revenue code as identified by the qualifier in SVC01-1.
C003-03
1339
Procedure Modifier
Optional
String (AN)
Min 2
Max 2
This identifies special circumstances related to the performance of the service, as defined by trading partners

C003-03 modifies the value in C003-02 and C003-08.
C003-04
1339
Procedure Modifier
Optional
String (AN)
Min 2
Max 2
This identifies special circumstances related to the performance of the service, as defined by trading partners

C003-04 modifies the value in C003-02 and C003-08.
C003-05
1339
Procedure Modifier
Optional
String (AN)
Min 2
Max 2
This identifies special circumstances related to the performance of the service, as defined by trading partners

C003-05 modifies the value in C003-02 and C003-08.
C003-06
1339
Procedure Modifier
Optional
String (AN)
Min 2
Max 2
This identifies special circumstances related to the performance of the service, as defined by trading partners

C003-06 modifies the value in C003-02 and C003-08.
SVC-02
782
Line Item Charge Amount
Required
Decimal number (R)
Min 1
Max 15
Monetary amount

SVC02 is the submitted service charge.
Usage notes
Use this monetary amount for the submitted service charge amount.
Decimal elements will be limited to a maximum length of 10 characters including reported or implied places for cents (implied value of 00 after the decimal point). This applies to all subsequent 782 elements.
SVC-03
782
Line Item Provider Payment Amount
Required
Decimal number (R)
Min 1
Max 15
Monetary amount

SVC03 is the amount paid this service.
Usage notes
Use this number for the service amount paid. The value in SVC03 must equal the value in SVC02 minus all monetary amounts in the subsequent CAS segments of this loop. See 1.10.2.1, Balancing, for additional information.
SVC-04
234
National Uniform Billing Committee Revenue Code
Optional
String (AN)
Min 1
Max 48
Identifying number for a product or service

SVC04 is the National Uniform Billing Committee Revenue Code.
Usage notes
If the original claim and adjudication only referenced an NUBC revenue code, that is supplied in SVC01 and this element is not used.
SVC-05
380
Units of Service Paid Count
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

SVC05 is the paid units of service.
Usage notes
If not present, the value is assumed to be one.
SVC-06
C003
Composite Medical Procedure Identifier
Optional
Max use 1
To identify a medical procedure by its standardized codes and applicable modifiers

- SVC06 is the original submitted medical procedure.
Usage notes
Required when the adjudicated procedure code provided in SVC01 is different from the submitted procedure code from the original claim. If not required by this implementation guide, do not send.

C003-01
235
Product or Service ID Qualifier
Required
Identifier (ID)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)

C003-01 qualifies C003-02 and C003-08.
Usage notes
The value in SVC06-1 qualifies the value in SVC06-2, SVC06-3, SVC06-4, SVC06-5, SVC06-6 and SVC06-7.
AD
American Dental Association Codes
ER
Jurisdiction Specific Procedure and Supply Codes
HC
Health Care Financing Administration Common Procedural Coding System (HCPCS) Codes
Because the CPT codes of the American Medical Association are also level 1 HCPCS codes, they are reported under the code HC.

HP
Health Insurance Prospective Payment System (HIPPS) Skilled Nursing Facility Rate Code
Medicare uses this code to reflect the Skilled Nursing Facility Group as well as the Home Health Agency Outpatient Prospective Payment System.

IV
Home Infusion EDI Coalition (HIEC) Product/Service Code
This code set is not allowed for use under HIPAA at the time of this writing. The qualifier can only be used 1) If a new rule names HIEC as an allowable code set under HIPAA. 2) For Property & Casualty claims/encounters that are not covered under HIPAA.

N4
National Drug Code in 5-4-2 Format
NU
National Uniform Billing Committee (NUBC) UB92 Codes
WK
Advanced Billing Concepts (ABC) Codes
This qualifier can only be used in transactions covere under HIPAA by parties registered in the pilot project and their trading partners.

C003-02
234
Procedure Code
Required
String (AN)
Min 1
Max 48
Identifying number for a product or service

If C003-08 is used, then C003-02 represents the beginning value in the range in which the code occurs.
C003-03
1339
Procedure Modifier
Optional
String (AN)
Min 2
Max 2
This identifies special circumstances related to the performance of the service, as defined by trading partners

C003-03 modifies the value in C003-02 and C003-08.
C003-04
1339
Procedure Modifier
Optional
String (AN)
Min 2
Max 2
This identifies special circumstances related to the performance of the service, as defined by trading partners

C003-04 modifies the value in C003-02 and C003-08.
C003-05
1339
Procedure Modifier
Optional
String (AN)
Min 2
Max 2
This identifies special circumstances related to the performance of the service, as defined by trading partners

C003-05 modifies the value in C003-02 and C003-08.
C003-06
1339
Procedure Modifier
Optional
String (AN)
Min 2
Max 2
This identifies special circumstances related to the performance of the service, as defined by trading partners

C003-06 modifies the value in C003-02 and C003-08.
C003-07
352
Procedure Code Description
Optional
String (AN)
Min 1
Max 80
A free-form description to clarify the related data elements and their content

C003-07 is the description of the procedure identified in C003-02.
SVC-07
380
Original Units of Service Count
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

SVC07 is the original submitted units of service.
DTM
0800
Service Date
Optional
Max use 2
To specify pertinent dates and times

Usage notes
Dates at the service line level apply only to the service line where they appear.
If used for inpatient claims and no service date was provided on the claim then report the through date from the claim level.
When claim dates are not provided, service dates are required for every service line.
When claim dates are provided, service dates are not required, but if used they override the claim dates for individual service lines.
Required when claim level Statement From or Through Dates are not supplied or the service dates are not the same as reported at the claim level. If not required by this implementation guide, may be provided at sender's discretion, but cannot be required by the receiver.
For retail pharmacy claims, the service date is equivalent to the prescription filled date.
For predeterminations, where there is no service date, the value of DTM02 must be 19000101. Use only when the CLP02 value is 25 - Predetermination Pricing Only - No Payment.
When payment is being made in advance of services, the use of future dates is allowed.
Example
DTM*151*20250519~
DTM-01
374
Date Time Qualifier
Required
Identifier (ID)
Code specifying type of date or time, or both date and time

150
Service Period Start
This qualifier is required for reporting the beginning of multi-day services. If not required by this implementation guide, do not send.

151
Service Period End
This qualifier is required for reporting the end of multi-day services. If not required by this implementation guide, do not send.

472
Service
This qualifier is required to indicate a single day service. If not required by this implementation guide, do not send.

DTM-02
373
Service Date
Required
Date (DT)
CCYYMMDD format
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year

CAS
0900
Service Adjustment
Optional
Max use 99
To supply adjustment reason codes and amounts as needed for an entire claim or for a particular service within the claim being paid

Usage notes
An example of this level of CAS is the reduction for the part of the service charge that exceeds the usual and customary charge for the service. See sections 1.10.2.1, Balancing, and 1.10.2.4, Claim Adjustment and Service Adjustment Segment Theory, for additional information.
Required when dollar amounts are being adjusted specific to the service or when the paid amount for a service line (SVC03) is different than the original submitted charge amount for the service (SVC02). If not required by this implementation guide, do not send.
A single CAS segment contains six repetitions of the "adjustment trio" composed of adjustment reason code, adjustment amount, and adjustment quantity. These six adjustment trios are used to report up to six adjustments related to a specific Claim Adjustment Group Code (CAS01). The six iterations (trios) of the Adjustment Reason Code related to the Specific Adjustment Group Code must be exhausted before repeating a second iteration of the CAS segment using the same Adjustment Group Code. The first adjustment is reported in the first adjustment trio (CAS02-CAS04). If there is a second non-zero adjustment, it is reported in the second adjustment trio (CAS05-CAS07), and so on through the sixth adjustment trio (CAS17-CAS19).
Example
CAS*PR*X*0000*000000000*X*000000*00000*XXXXX*00000000000*0000*X*00*00000000*XX*0000*000*XXXXX*000*00000000~
If Adjustment Reason Code (CAS-05) is present, then at least one of Adjustment Amount (CAS-06) or Adjustment Quantity (CAS-07) is required
If Adjustment Amount (CAS-06) is present, then Adjustment Reason Code (CAS-05) is required
If Adjustment Quantity (CAS-07) is present, then Adjustment Reason Code (CAS-05) is required
If Adjustment Reason Code (CAS-08) is present, then at least one of Adjustment Amount (CAS-09) or Adjustment Quantity (CAS-10) is required
If Adjustment Amount (CAS-09) is present, then Adjustment Reason Code (CAS-08) is required
If Adjustment Quantity (CAS-10) is present, then Adjustment Reason Code (CAS-08) is required
If Adjustment Reason Code (CAS-11) is present, then at least one of Adjustment Amount (CAS-12) or Adjustment Quantity (CAS-13) is required
If Adjustment Amount (CAS-12) is present, then Adjustment Reason Code (CAS-11) is required
If Adjustment Quantity (CAS-13) is present, then Adjustment Reason Code (CAS-11) is required
If Adjustment Reason Code (CAS-14) is present, then at least one of Adjustment Amount (CAS-15) or Adjustment Quantity (CAS-16) is required
If Adjustment Amount (CAS-15) is present, then Adjustment Reason Code (CAS-14) is required
If Adjustment Quantity (CAS-16) is present, then Adjustment Reason Code (CAS-14) is required
If Adjustment Reason Code (CAS-17) is present, then at least one of Adjustment Amount (CAS-18) or Adjustment Quantity (CAS-19) is required
If Adjustment Amount (CAS-18) is present, then Adjustment Reason Code (CAS-17) is required
If Adjustment Quantity (CAS-19) is present, then Adjustment Reason Code (CAS-17) is required
CAS-01
1033
Claim Adjustment Group Code
Required
Identifier (ID)
Code identifying the general category of payment adjustment

Usage notes
Evaluate the usage of group codes in CAS01 based on the following order for their applicability to a set of one or more adjustments: PR, CO, PI, OA. See 1.10.2.4, Claim Adjustment and Service Adjustment Segment Theory, for additional information. (Note: This does not mean that the adjustments must be reported in this order.)
CO
Contractual Obligations
Use this code when a joint payer/payee agreement or a regulatory requirement has resulted in an adjustment.

OA
Other adjustments
Avoid using the Other Adjustment Group Code (OA) except for business situations described in sections 1.10.2.6, 1.10.2.7 and 1.10.2.13.

PI
Payor Initiated Reductions
Use this code when, in the opinion of the payer, the adjustment is not the responsibility of the patient, but there is no supporting contract between the provider and the payer (i.e., medical review or professional review organization adjustments).

PR
Patient Responsibility
CAS-02
1034
Adjustment Reason Code
Required
Identifier (ID)
Min 1
Max 5
Code identifying the detailed reason the adjustment was made

Usage notes
Required to report a non-zero adjustment applied at the service level for the claim adjustment group code reported in CAS01.
CAS-03
782
Adjustment Amount
Required
Decimal number (R)
Min 1
Max 15
Monetary amount

CAS03 is the amount of adjustment.
Usage notes
Use this monetary amount for the adjustment amount. A negative amount increases the payment, and a positive amount decreases the payment contained in SVC03 and CLP04.
Decimal elements will be limited to a maximum length of 10 characters including reported or implied places for cents (implied value of 00 after the decimal point). This applies to all subsequent 782 elements.
CAS-04
380
Adjustment Quantity
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

CAS04 is the units of service being adjusted.
Usage notes
A positive number decreases paid units, and a negative value increases paid units.
CAS-05
1034
Adjustment Reason Code
Optional
Identifier (ID)
Min 1
Max 5
Code identifying the detailed reason the adjustment was made

Usage notes
See CAS02.
CAS-06
782
Adjustment Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

CAS06 is the amount of the adjustment.
Usage notes
See CAS03.
CAS-07
380
Adjustment Quantity
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

CAS07 is the units of service being adjusted.
Usage notes
See CAS04.
CAS-08
1034
Adjustment Reason Code
Optional
Identifier (ID)
Min 1
Max 5
Code identifying the detailed reason the adjustment was made

Usage notes
See CAS02.
CAS-09
782
Adjustment Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

CAS09 is the amount of the adjustment.
Usage notes
See CAS03.
CAS-10
380
Adjustment Quantity
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

CAS10 is the units of service being adjusted.
Usage notes
See CAS04.
CAS-11
1034
Adjustment Reason Code
Optional
Identifier (ID)
Min 1
Max 5
Code identifying the detailed reason the adjustment was made

Usage notes
See CAS02.
CAS-12
782
Adjustment Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

CAS12 is the amount of the adjustment.
Usage notes
See CAS03.
CAS-13
380
Adjustment Quantity
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

CAS13 is the units of service being adjusted.
Usage notes
See CAS04.
CAS-14
1034
Adjustment Reason Code
Optional
Identifier (ID)
Min 1
Max 5
Code identifying the detailed reason the adjustment was made

Usage notes
See CAS02.
CAS-15
782
Adjustment Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

CAS15 is the amount of the adjustment.
Usage notes
See CAS03.
CAS-16
380
Adjustment Quantity
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

CAS16 is the units of service being adjusted.
Usage notes
See CAS04.
CAS-17
1034
Adjustment Reason Code
Optional
Identifier (ID)
Min 1
Max 5
Code identifying the detailed reason the adjustment was made

Usage notes
See CAS02.
CAS-18
782
Adjustment Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

CAS18 is the amount of the adjustment.
Usage notes
See CAS03.
CAS-19
380
Adjustment Quantity
Optional
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

CAS19 is the units of service being adjusted.
Usage notes
See CAS04.
REF
1000
Service Identification
Optional
Max use 8
To specify identifying information

Usage notes
Required when related service specific reference identifiers were used in the process of adjudicating this service. If not required by this implementation guide, do not send.
Example
REF*E9*XXXX~
Variants (all may be used)
REFLine Item Control Number
REFRendering Provider Information
REFHealthCare Policy Identification
REF-01
128
Reference Identification Qualifier
Required
Identifier (ID)
Code qualifying the Reference Identification

1S
Ambulatory Patient Group (APG) Number
APC
Ambulatory Payment Classification
BB
Authorization Number
E9
Attachment Code
G1
Prior Authorization Number
G3
Predetermination of Benefits Identification Number
LU
Location Number
This is the Payer's identification for the provider location. This is REQUIRED when the specific site of service affected the payment of the claim.

RB
Rate code number
Rate Code Number reflects Ambulatory Surgical Center (ASC) rate for Medicare, either 0, 50, 100 or 150%.

REF-02
127
Provider Identifier
Required
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

REF
1000
Line Item Control Number
Optional
Max use 1
To specify identifying information

Usage notes
This is the Line Item Control Number submitted in the 837, which is utilized by the provider for tracking purposes. See section 1.10.2.11 and 1.10.2.14.1 for additional information on usage with split claims or services. Note - the value in REF02 can include alpha characters.
Required when a Line Item Control Number was received on the original claim or when claim or service line splitting has occurred. If not required by this implementation guide, do not send.
Example
REF*6R*XXX~
Variants (all may be used)
REFService Identification
REFRendering Provider Information
REFHealthCare Policy Identification
REF-01
128
Reference Identification Qualifier
Required
Identifier (ID)
Code qualifying the Reference Identification

6R
Provider Control Number
REF-02
127
Line Item Control Number
Required
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

REF
1000
Rendering Provider Information
Optional
Max use 10
To specify identifying information

Usage notes
Required when the rendering provider for this service is different than the rendering provider applicable at the claim level. If not required by this implementation guide, do not send.
Example
REF*1J*XXXX~
Variants (all may be used)
REFService Identification
REFLine Item Control Number
REFHealthCare Policy Identification
REF-01
128
Reference Identification Qualifier
Required
Identifier (ID)
Code qualifying the Reference Identification

0B
State License Number
1A
Blue Cross Provider Number
1B
Blue Shield Provider Number
1C
Medicare Provider Number
1D
Medicaid Provider Number
1G
Provider UPIN Number
1H
CHAMPUS Identification Number
1J
Facility ID Number
D3
National Council for Prescription Drug Programs Pharmacy Number
G2
Provider Commercial Number
HPI
Centers for Medicare and Medicaid Services National Provider Identifier
This qualifier is REQUIRED when the National Provider Identifier is mandated for use and the provider is a covered health care provider under that mandate.

SY
Social Security Number
TJ
Federal Taxpayer's Identification Number
REF-02
127
Rendering Provider Identifier
Required
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

REF
1000
HealthCare Policy Identification
Optional
Max use 5
To specify identifying information

Usage notes
Required when;
The payment is adjusted in accordance with the Payer's published Healthcare Policy Code list and
A Claim Adjustment Reason Code identified by the notation, "refer to 835 Healthcare Policy identification segment", in the Claim Adjustment Reason Code List is present in a related CAS segment and
The payer has a published enumerated healthcare policy code list available to healthcare providers via an un-secure public website and
The payer wishes to supply this policy detail to reduce provider inquiries.
If not required by this implementation guide, may be provided at the sender's discretion, but cannot be required by the receiver.

Healthcare Policy - A clinical/statutory rule use to determine claim adjudication that cannot be explained by the sole use of a claim adjustment reason code in the CAS segment and Remittance Advise Remark code when appropriate.
The term Healthcare Policy is intended to include Medical Review Policy, Dental Policy Review, Property and Casualty Policies, Workers Comp Policies and Pharmacy Policies for example Medicare LMRP's.( Local Medicare Review policies) and NCD (National Coverage Determinations).
This policy segment must not be used to provide a proprietary explanation code or reason for adjustment.
Supply the Healthcare policy identifier in REF02 as provided by the payer's published Healthcare policy code list. This policy code will be used to explain the policy used to process the claim which resulted in the adjusted payment.
If this segment is used, the PER (Payer Web Site) segment is required to provide an un-secure WEB contact point where the provider can access the payer's enumerated, published healthcare policy.
Example
REF*0K*XX~
Variants (all may be used)
REFService Identification
REFLine Item Control Number
REFRendering Provider Information
REF-01
128
Reference Identification Qualifier
Required
Identifier (ID)
Code qualifying the Reference Identification

0K
Policy Form Identifying Number
REF-02
127
Healthcare Policy Identification
Required
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

AMT
1100
Service Supplemental Amount
Optional
Max use 9
To indicate the total monetary amount

Usage notes
This segment is used to convey information only. It is not part of the financial balancing of the 835.
Required when the value of any specific amount identified by the AMT01 qualifier is non-zero. If not required by this implementation guide, do not send.
Example
AMT*T*000000~
AMT-01
522
Amount Qualifier Code
Required
Identifier (ID)
Code to qualify amount

B6
Allowed - Actual
Allowed amount is the amount the payer deems payable prior to considering patient responsibility.

KH
Deduction Amount
Late Filing Reduction

T
Tax
T2
Total Claim Before Taxes
Use this monetary amount for the service charge before taxes. This is only used when there is an applicable tax amount on this service.

ZK
Federal Medicare or Medicaid Payment Mandate - Category 1
ZL
Federal Medicare or Medicaid Payment Mandate - Category 2
ZM
Federal Medicare or Medicaid Payment Mandate - Category 3
ZN
Federal Medicare or Medicaid Payment Mandate - Category 4
ZO
Federal Medicare or Medicaid Payment Mandate - Category 5
AMT-02
782
Service Supplemental Amount
Required
Decimal number (R)
Min 1
Max 15
Monetary amount

Usage notes
Decimal elements will be limited to a maximum length of 10 characters including reported or implied places for cents (implied value of 00 after the decimal point). This applies to all subsequent 782 elements.
QTY
1200
Service Supplemental Quantity
Optional
Max use 6
To specify quantity information

Usage notes
Use this segment to convey information only. It is not part of the financial balancing of the 835.
Required when new Federal Medicare or Medicaid mandates require Quantity counts and value of specific quantities identified in the QTY01 qualifier are non-zero. If not required by this implementation guide, do not send.
Example
QTY*ZK*00000000~
QTY-01
673
Quantity Qualifier
Required
Identifier (ID)
Code specifying the type of quantity

ZK
Federal Medicare or Medicaid Payment Mandate - Category 1
ZL
Federal Medicare or Medicaid Payment Mandate - Category 2
ZM
Federal Medicare or Medicaid Payment Mandate - Category 3
ZN
Federal Medicare or Medicaid Payment Mandate - Category 4
ZO
Federal Medicare or Medicaid Payment Mandate - Category 5
QTY-02
380
Service Supplemental Quantity Count
Required
Decimal number (R)
Min 1
Max 15
Numeric value of quantity

LQ
1300
Health Care Remark Codes
Optional
Max use 99
To identify standard industry codes

Usage notes
Use this segment to provide informational remarks only. This segment has no impact on the actual payment. Changes in claim payment amounts are provided in the CAS segments.
Required when remark codes or NCPDP Reject/Payment codes are necessary for the provider to fully understand the adjudication message for a given service line. If not required by this implementation guide, may be provided at the sender's discretion, but cannot be required by the receiver.
Example
LQ*RX*XXX~
If Code List Qualifier Code (LQ-01) is present, then Remark Code (LQ-02) is required
LQ-01
1270
Code List Qualifier Code
Required
Identifier (ID)
Code identifying a specific industry code list

HE
Claim Payment Remark Codes
RX
National Council for Prescription Drug Programs Reject/Payment Codes
LQ-02
1271
Remark Code
Required
String (AN)
Min 1
Max 30
Code indicating a code from a specific industry code list

Summary
PLB
0100
Provider Adjustment
Optional
Max use >1
To convey provider level adjustment information for debit or credit transactions such as, accelerated payments, cost report settlements for a fiscal year and timeliness report penalties unrelated to a specific claim or service

Usage notes
These adjustments can either decrease the payment (a positive number) or increase the payment (a negative number). Zero dollar adjustments are not allowed. Some examples of PLB adjustments are a Periodic Interim Payment (loans and loan repayment) or a capitation payment. Multiple adjustments can be placed in one PLB segment, grouped by the provider identified in PLB01 and the period identified in PLB02. Although the PLB reference numbers are not standardized, refer to 1.10.2.9 (Interest and Prompt Payment Discounts), 1.10.2.10 (Capitation and Related Payments or Adjustments), 1.10.2.12 (Balance Forward Processing), 1.10.2.16 (Post Payment Recovery) and 1.10.2.17 (Claim Overpayment Recovery) for code suggestions and usage guidelines.
The codes and notations under PLB03 and its components apply equally to PLB05, 07, 09, 11 and 13.
Required when reporting adjustments to the actual payment that are NOT specific to a particular claim or service. If not required by this implementation guide, do not send.
Example
PLB*XXX*20250519*72>XXXXX*0000000*XX>X*00000000000000*XX>XX*000000000000*XX>XXX*000000000000*XX>XXXX*000000000000*XX>XX*0000~
If either Adjustment Identifier (PLB-05) or Provider Adjustment Amount (PLB-06) is present, then the other is required
If either Adjustment Identifier (PLB-07) or Provider Adjustment Amount (PLB-08) is present, then the other is required
If either Adjustment Identifier (PLB-09) or Provider Adjustment Amount (PLB-10) is present, then the other is required
If either Adjustment Identifier (PLB-11) or Provider Adjustment Amount (PLB-12) is present, then the other is required
If either Adjustment Identifier (PLB-13) or Provider Adjustment Amount (PLB-14) is present, then the other is required
PLB-01
127
Provider Identifier
Required
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

PLB01 is the provider number assigned by the payer.
Usage notes
When the National Provider Identifier (NPI) is mandated and the provider is a covered health care provider under that mandate, this must be the NPI assigned to the provider.
Until the NPI is mandated, this is the provider identifier as assigned by the payer.
PLB-02
373
Fiscal Period Date
Required
Date (DT)
CCYYMMDD format
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year

PLB02 is the last day of the provider's fiscal year.
Usage notes
This is the last day of the provider's fiscal year. If the end of the provider's fiscal year is not known by the payer, use December 31st of the current year.
PLB-03
C042
Adjustment Identifier
Required
Max use 1
To provide the category and identifying reference information for an adjustment

- PLB03 is the adjustment information as defined by the payer.
C042-01
426
Adjustment Reason Code
Required
Identifier (ID)
Code indicating reason for debit or credit memo or adjustment to invoice, debit or credit memo, or payment

50
Late Charge
This is the Late Claim Filing Penalty or Medicare Late Cost Report Penalty.

51
Interest Penalty Charge
This is the interest assessment for late filing.

72
Authorized Return
This is the provider refund adjustment. This adjustment acknowledges a refund received from a provider for previous overpayment. PLB03-2 must always contain an identifying reference number when the value is used. PLB04 must contain a negative value. This adjustment must always be offset by some other PLB adjustment referring to the original refund request or reason. For balancing purposes, the amount related to this adjustment reason code must be directly offset.

90
Early Payment Allowance
AH
Origination Fee
This is the claim transmission fee. This is used for transmission fees that are not specific to or dependent upon individual claims.

AM
Applied to Borrower's Account
See 1.10.2.10, Capitation and Related Payments or Adjustments, for additional information. Use this code to identify the loan repayment amount.

This is capitation specific.

AP
Acceleration of Benefits
This is the accelerated payment amount or withholding. Withholding or payment identification is indicated by the sign of the amount in PLB04. A positive value represents a withholding. A negative value represents a payment.

B2
Rebate
This adjustment code applies when a provider has remitted an overpayment to a health plan in excess of the amount requested by the health plan. The amount accepted by the health plan is reported using code 72 (Authorized Return) and offset by the amount with code WO (Overpayment Recovery). The excess returned by the provider is reported as a negative amount using code B2, returning the excess funds to the provider.

B3
Recovery Allowance
This represents the check received from the provider for overpayments generated by payments from other payers. This code differs from the provider refund adjustment identified with code 72. This adjustment must always be offset by some other PLB adjustment referring to the original refund request or reason. For balancing purposes, the amount related to this adjustment reason code must be directly offset.

BD
Bad Debt Adjustment
This is the bad debt passthrough.

BN
Bonus
This is capitation specific. See 1.10.2.10, Capitation and Related Payments or Adjustments, for additional information.

C5
Temporary Allowance
This is the tentative adjustment.

CR
Capitation Interest
This is capitation specific. See 1.10.2.10, Capitation and Related Payments or Adjustments, for additional information.

CS
Adjustment
Provide supporting identification information in PLB03-2.

CT
Capitation Payment
This is capitation specific. See 1.10.2.10, Capitation and Related Payments or Adjustments, for additional information.

CV
Capital Passthru
CW
Certified Registered Nurse Anesthetist Passthru
DM
Direct Medical Education Passthru
E3
Withholding
See 1.10.2.10, Capitation and Related Payments or Adjustments, for additional information.

FB
Forwarding Balance
This is the balance forward. A negative value in PLB04 represents a balance moving forward to a future payment advice. A positive value represents a balance being applied from a previous payment advice. A reference number must be supplied in PLB03-2 for tracking purposes. See 1.10.2.12, Balance Forward Processing, for further information.

FC
Fund Allocation
This is capitation specific. See 1.10.2.10, Capitation and Related Payments or Adjustments, for additional information. The specific fund must be identified in PLB03-2.

GO
Graduate Medical Education Passthru
HM
Hemophilia Clotting Factor Supplement
IP
Incentive Premium Payment
This is capitation specific. See 1.10.2.10, Capitation and Related Payments or Adjustments, for additional information.

IR
Internal Revenue Service Withholding
IS
Interim Settlement
This is the interim rate lump sum adjustment.

J1
Nonreimbursable
This offsets the claim or service level data that reflects what could be paid if not for demonstration program or other limitation that prevents issuance of payment.

L3
Penalty
This is the capitation-related penalty. Withholding or release is identified by the sign in PLB04. See 1.10.2.10, Capitation and Related Payments or Adjustments, for additional information.

L6
Interest Owed
This is the interest paid on claims in this 835. Support the amounts related to this adjustment by 2-062 AMT amounts, where AMT01 is "I".

LE
Levy
IRS Levy

LS
Lump Sum
This is the disproportionate share adjustment, indirect medical education passthrough, non-physician passthrough, passthrough lump sum adjustment, or other passthrough amount. The specific type of lump sum adjustment must be identified in PLB03-2.

OA
Organ Acquisition Passthru
OB
Offset for Affiliated Providers
Identification of the affiliated providers must be made on PLB03-2.

PI
Periodic Interim Payment
This is the periodic interim lump sum payments and reductions (PIP). The payments are made to a provider at the beginning of some period in advance of claims. These payments are advances on the expected claims for the period. The reductions are the recovery of actual claims payments during the period. For instance, when a provider has a PIP payment, claims within this remittance advice covered by that payment would be offset using this code to remove the claim payment from the current check. The sign of the amount in PLB04 determines whether this is a payment (negative) or reduction (positive).

This payment and recoupment is effectively a loan to the provider and loan repayment.

See section 1.10.2.5, Advance Payments and Reconciliation, for additional information.

PL
Payment Final
This is the final settlement.

RA
Retro-activity Adjustment
This is capitation specific. See 1.10.2.10, Capitation and Related Payments or Adjustments, for additional information.

RE
Return on Equity
SL
Student Loan Repayment
TL
Third Party Liability
This is capitation specific. See 1.10.2.10, Capitation and Related Payments or Adjustments, for additional information.

WO
Overpayment Recovery
This is the recovery of previous overpayment. An identifying number must be provided in PLB03-2. See the notes on codes 72 and B3 for additional information about balancing against a provider refund.

WU
Unspecified Recovery
Medicare is currently using this code to represent penalty collections withheld for the IRS (an outside source).

C042-02
127
Provider Adjustment Identifier
Optional
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

Usage notes
Use when necessary to assist the receiver in identifying, tracking or reconcilling the adjustment. See sections 1.10.2.10 (Capitation and Related Payments), 1.10.2.5 (Advanced Payments and Reconciliation) and 1.10.2.12 (Balance Forward Processing) for further information.
PLB-04
782
Provider Adjustment Amount
Required
Decimal number (R)
Min 1
Max 15
Monetary amount

PLB04 is the adjustment amount.
Usage notes
This is the adjustment amount for the preceding adjustment reason.
Decimal elements will be limited to a maximum length of 10 characters including reported or implied places for cents (implied value of 00 after the decimal point). This applies to all subsequent 782 elements.
PLB-05
C042
Adjustment Identifier
Optional
Max use 1
To provide the category and identifying reference information for an adjustment

- PLB05 is the adjustment information as defined by the payer.
Usage notes
Required when an additional adjustment not already reported applies to this remittance advice. If not required by this implementation guide, do not send.

C042-01
426
Adjustment Reason Code
Required
Identifier (ID)
Min 2
Max 2
Code indicating reason for debit or credit memo or adjustment to invoice, debit or credit memo, or payment

C042-02
127
Provider Adjustment Identifier
Optional
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

PLB-06
782
Provider Adjustment Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

PLB06 is the adjustment amount.
Usage notes
This is the adjustment amount for the preceding adjustment reason.
PLB-07
C042
Adjustment Identifier
Optional
Max use 1
To provide the category and identifying reference information for an adjustment

- PLB07 is adjustment information as defined by the payer.
Usage notes
Required when an additional adjustment not already reported applies to this remittance advice. If not required by this implementation guide, do not send.

C042-01
426
Adjustment Reason Code
Required
Identifier (ID)
Min 2
Max 2
Code indicating reason for debit or credit memo or adjustment to invoice, debit or credit memo, or payment

C042-02
127
Provider Adjustment Identifier
Optional
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

PLB-08
782
Provider Adjustment Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

PLB08 is the adjustment amount.
Usage notes
This is the adjustment amount for the preceding adjustment reason.
PLB-09
C042
Adjustment Identifier
Optional
Max use 1
To provide the category and identifying reference information for an adjustment

- PLB09 is adjustment information as defined by the payer.
Usage notes
Required when an additional adjustment not already reported applies to this remittance advice. If not required by this implementation guide, do not send.

C042-01
426
Adjustment Reason Code
Required
Identifier (ID)
Min 2
Max 2
Code indicating reason for debit or credit memo or adjustment to invoice, debit or credit memo, or payment

C042-02
127
Provider Adjustment Identifier
Optional
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

PLB-10
782
Provider Adjustment Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

PLB10 is the adjustment amount.
Usage notes
This is the adjustment amount for the preceding adjustment reason.
PLB-11
C042
Adjustment Identifier
Optional
Max use 1
To provide the category and identifying reference information for an adjustment

- PLB11 is adjustment information as defined by the payer.
Usage notes
Required when an additional adjustment not already reported applies to this remittance advice. If not required by this implementation guide, do not send.

C042-01
426
Adjustment Reason Code
Required
Identifier (ID)
Min 2
Max 2
Code indicating reason for debit or credit memo or adjustment to invoice, debit or credit memo, or payment

C042-02
127
Provider Adjustment Identifier
Optional
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

PLB-12
782
Provider Adjustment Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

PLB12 is the adjustment amount.
Usage notes
This is the adjustment amount for the preceding adjustment reason.
PLB-13
C042
Adjustment Identifier
Optional
Max use 1
To provide the category and identifying reference information for an adjustment

- PLB13 is adjustment information as defined by the payer.
Usage notes
Required when an additional adjustment not already reported applies to this remittance advice. If not required by this implementation guide, do not send.

C042-01
426
Adjustment Reason Code
Required
Identifier (ID)
Min 2
Max 2
Code indicating reason for debit or credit memo or adjustment to invoice, debit or credit memo, or payment

C042-02
127
Provider Adjustment Identifier
Optional
String (AN)
Min 1
Max 50
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier

PLB-14
782
Provider Adjustment Amount
Optional
Decimal number (R)
Min 1
Max 15
Monetary amount

PLB14 is the adjustment amount.
Usage notes
This is the adjustment amount for the preceding adjustment reason.
SE
0200
Transaction Set Trailer
Required
Max use 1
To indicate the end of the transaction set and provide the count of the transmitted segments (including the beginning (ST) and ending (SE) segments)

Example
SE*00*0001~
SE-01
96
Transaction Segment Count
Required
Numeric (N0)
Min 1
Max 10
Total number of segments included in a transaction set including ST and SE segments

SE-02
329
Transaction Set Control Number
Required
Numeric (N)
Min 4
Max 9
Identifying control number that must be unique within the transaction set functional group assigned by the originator for a transaction set

Usage notes
The Transaction Set Control Numbers in ST02 and SE02 must be identical. The originator assigns the Transaction Set Control Number, which must be unique within a functional group (GS-GE). This unique number also aids in error resolution research.
GE
Functional Group Trailer
Required
Max use 1
To indicate the end of a functional group and to provide control information

Example
GE*000000*0000000~
GE-01
97
Number of Transaction Sets Included
Required
Numeric (N0)
Min 1
Max 6
Total number of transaction sets included in the functional group or interchange (transmission) group terminated by the trailer containing this data element

GE-02
28
Group Control Number
Required
Numeric (N0)
Min 1
Max 9
Assigned number originated and maintained by the sender

IEA
Interchange Control Trailer
Required
Max use 1
To define the end of an interchange of zero or more functional groups and interchange-related control segments

Example
IEA*00*000000000~
IEA-01
I16
Number of Included Functional Groups
Required
Numeric (N0)
Min 1
Max 5
A count of the number of functional groups included in an interchange

IEA-02
I12
Interchange Control Number
Required
Numeric (N0)
Min 9
Max 9
A control number assigned by the interchange sender

Stedi is a registered trademark of Stedi, Inc. All names, logos, and brands of third parties listed on this page are trademarks of their respective owners (including “X12”, which is a trademark of X12 Incorporated). Stedi, Inc. and its products and services are not endorsed by, sponsored by, or affiliated with these third parties. Use of these names, logos, and brands is for identification purposes only, and does not imply any such endorsement, sponsorship, or affiliation.